use crate::document::document_impl::TreeIterator;
use crate::errors::Error;
use crate::parser::query::{Condition, Query, SearchType};
use gosub_interface::config::HasDocument;
use gosub_interface::document::Document;

use gosub_interface::node::ClassList;
use gosub_interface::node::ElementDataType;
use gosub_interface::node::Node;
use gosub_shared::node::NodeId;

pub struct DocumentQuery<C: HasDocument> {
    _phantom: std::marker::PhantomData<C>,
}

impl<C: HasDocument> DocumentQuery<C> {
    /// Perform a single query against the document.
    /// If query search type is uninitialized, returns an error.
    /// Otherwise, returns a vector of `NodeIds` that match the predicate in tree order (preorder depth-first.)
    pub fn query(doc: &C::Document, query: &Query) -> gosub_shared::types::Result<Vec<NodeId>> {
        if query.search_type == SearchType::Uninitialized {
            return Err(Error::Query("Query predicate is uninitialized".to_owned()).into());
        }

        let tree_iterator = TreeIterator::<C>::new(doc);

        let mut found_ids = Vec::new();
        for current_node_id in tree_iterator {
            let mut predicate_result: bool = true;
            for condition in &query.conditions {
                if !Self::matches_query_condition(doc, &current_node_id, condition) {
                    predicate_result = false;
                    break;
                }
            }

            if predicate_result {
                found_ids.push(current_node_id);
                if query.search_type == SearchType::FindFirst {
                    return Ok(found_ids);
                }
            }
        }

        Ok(found_ids)
    }

    /// Check if a given node's children contain a certain tag name
    pub fn contains_child_tag(doc: &C::Document, node_id: NodeId, tag: &str) -> bool {
        if let Some(node) = doc.node_by_id(node_id) {
            for child_id in &node.children().to_vec() {
                if let Some(child) = doc.node_by_id(*child_id) {
                    if let Some(data) = child.get_element_data() {
                        if data.name() == tag {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn matches_query_condition(doc: &C::Document, current_node_id: &NodeId, condition: &Condition) -> bool {
        let Some(current_node) = doc.node_by_id(*current_node_id) else {
            return false;
        };

        match condition {
            Condition::EqualsTag(tag) => {
                let Some(current_node_data) = current_node.get_element_data() else {
                    return false;
                };
                current_node_data.name() == *tag
            }
            Condition::EqualsId(id) => {
                let Some(current_node_data) = current_node.get_element_data() else {
                    return false;
                };

                if let Some(id_attr) = current_node_data.attributes().get("id") {
                    return *id_attr == *id;
                }

                false
            }
            Condition::ContainsClass(class_name) => {
                let Some(current_node_data) = current_node.get_element_data() else {
                    return false;
                };

                current_node_data.classlist().contains(class_name)
            }
            Condition::ContainsAttribute(attribute) => {
                let Some(current_node_data) = current_node.get_element_data() else {
                    return false;
                };

                current_node_data.attributes().contains_key(attribute)
            }
            Condition::ContainsChildTag(child_tag) => Self::contains_child_tag(doc, current_node.id(), child_tag),
            Condition::HasParentTag(parent_tag) => {
                if let Some(parent_id) = current_node.parent_id() {
                    // making an assumption here that the parent node is actually valid
                    let parent = doc.node_by_id(parent_id).unwrap();
                    if let Some(parent_data) = parent.get_element_data() {
                        return parent_data.name() == *parent_tag;
                    }
                    return false;
                }

                false
            }
        }
    }
}
