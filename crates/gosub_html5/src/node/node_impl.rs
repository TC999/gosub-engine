use crate::document::document_impl::DocumentImpl;
use crate::node::data::comment::CommentData;
use crate::node::data::doctype::DocTypeData;
use crate::node::data::document::DocumentData;
use crate::node::data::element::ElementData;
use crate::node::data::text::TextData;
use core::fmt::Debug;
use gosub_interface::config::HasDocument;

use gosub_interface::node::{Node, NodeData, NodeType, QuirksMode};
use gosub_shared::byte_stream::Location;
use gosub_shared::node::NodeId;
use std::collections::HashMap;

/// Implementation of the `NodeDataType` trait
#[derive(Debug, Clone, PartialEq)]
pub enum NodeDataTypeInternal<C: HasDocument> {
    /// Represents a document
    Document(DocumentData),
    // Represents a doctype
    DocType(DocTypeData),
    /// Represents a text
    Text(TextData),
    /// Represents a comment
    Comment(CommentData),
    /// Represents an element
    Element(ElementData<C>),
}

/// Node structure that resembles a DOM node
pub struct NodeImpl<C: HasDocument> {
    /// ID of the node, 0 is always the root / document node
    pub id: NodeId,
    /// parent of the node, if any
    pub parent: Option<NodeId>,
    /// any children of the node
    pub children: Vec<NodeId>,
    /// actual data of the node
    pub data: NodeDataTypeInternal<C>,
    // Returns true when the given node is registered into the document arena
    pub registered: bool,
    // Location of the node in the source code
    pub location: Location,
}

impl<C: HasDocument<Document = DocumentImpl<C>>> Node<C> for NodeImpl<C> {
    type DocumentData = DocumentData;
    type DocTypeData = DocTypeData;
    type TextData = TextData;
    type CommentData = CommentData;
    type ElementData = ElementData<C>;

    /// Creates a new node based on the original node, but without any attachments (childs, parents) or a node-id
    fn new_from_node(org_node: &Self) -> Self {
        let (id, parent, children, registered) = <_>::default();

        Self {
            id,
            parent,
            children,
            data: org_node.data.clone(),
            registered,
            location: org_node.location(),
        }
    }

    fn id(&self) -> NodeId {
        self.id
    }

    fn set_id(&mut self, id: NodeId) {
        self.id = id;
    }

    fn location(&self) -> Location {
        self.location
    }

    fn parent_id(&self) -> Option<NodeId> {
        self.parent
    }

    fn set_parent(&mut self, parent_id: Option<NodeId>) {
        self.parent = parent_id;
    }

    fn set_registered(&mut self, registered: bool) {
        self.registered = registered;
    }

    fn is_registered(&self) -> bool {
        self.registered
    }

    fn is_root(&self) -> bool {
        self.parent_id().is_none()
    }

    fn children(&self) -> &[NodeId] {
        self.children.as_slice()
    }

    fn type_of(&self) -> NodeType {
        match self.data {
            NodeDataTypeInternal::Document(_) => NodeType::DocumentNode,
            NodeDataTypeInternal::DocType(_) => NodeType::DocTypeNode,
            NodeDataTypeInternal::Text(_) => NodeType::TextNode,
            NodeDataTypeInternal::Comment(_) => NodeType::CommentNode,
            NodeDataTypeInternal::Element(_) => NodeType::ElementNode,
        }
    }

    fn is_element_node(&self) -> bool {
        self.type_of() == NodeType::ElementNode
    }

    fn get_element_data(&self) -> Option<&Self::ElementData> {
        if let NodeDataTypeInternal::Element(data) = &self.data {
            return Some(data);
        }
        None
    }

    fn get_element_data_mut(&mut self) -> Option<&mut ElementData<C>> {
        if let NodeDataTypeInternal::Element(data) = &mut self.data {
            return Some(data);
        }
        None
    }

    fn is_text_node(&self) -> bool {
        matches!(self.data, NodeDataTypeInternal::Text(_))
    }

    fn get_text_data(&self) -> Option<&Self::TextData> {
        if let NodeDataTypeInternal::Text(data) = &self.data {
            return Some(data);
        }
        None
    }

    fn get_text_data_mut(&mut self) -> Option<&mut TextData> {
        if let NodeDataTypeInternal::Text(data) = &mut self.data {
            return Some(data);
        }
        None
    }

    fn get_comment_data(&self) -> Option<&Self::CommentData> {
        if let NodeDataTypeInternal::Comment(data) = &self.data {
            return Some(data);
        }
        None
    }

    fn get_doctype_data(&self) -> Option<&Self::DocTypeData> {
        if let NodeDataTypeInternal::DocType(data) = &self.data {
            return Some(data);
        }
        None
    }

    fn remove(&mut self, node_id: NodeId) {
        self.children.retain(|x| x != &node_id);
    }

    fn insert(&mut self, node_id: NodeId, idx: usize) {
        self.children.insert(idx, node_id);
    }

    fn push(&mut self, node_id: NodeId) {
        self.children.push(node_id);
    }

    fn data(&self) -> NodeData<'_, C> {
        match self.data {
            NodeDataTypeInternal::Document(ref data) => NodeData::Document(data),
            NodeDataTypeInternal::DocType(ref data) => NodeData::DocType(data),
            NodeDataTypeInternal::Text(ref data) => NodeData::Text(data),
            NodeDataTypeInternal::Comment(ref data) => NodeData::Comment(data),
            NodeDataTypeInternal::Element(ref data) => NodeData::Element(data),
        }
    }
}

impl<C: HasDocument<Document = DocumentImpl<C>>> PartialEq for NodeImpl<C> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id()
    }
}

impl<C: HasDocument> Debug for NodeImpl<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Node");
        debug.field("id", &self.id);
        debug.field("parent", &self.parent);
        debug.field("children", &self.children);
        // @todo: add element/doctype etc data
        debug.finish_non_exhaustive()
    }
}

impl<C: HasDocument> Clone for NodeImpl<C> {
    fn clone(&self) -> Self {
        NodeImpl {
            id: self.id,
            parent: self.parent,
            children: self.children.clone(),
            data: self.data.clone(),
            registered: self.registered,
            location: self.location,
        }
    }
}

impl<C: HasDocument> NodeImpl<C> {
    /// create a new `Node`
    #[must_use]
    pub fn new(location: Location, data: NodeDataTypeInternal<C>) -> Self {
        let (id, parent, children, registered) = <_>::default();

        Self {
            id,
            parent,
            children,
            data,
            registered,
            location,
        }
    }

    /// Create a new document node
    #[must_use]
    pub fn new_document(location: Location, quirks_mode: QuirksMode) -> Self {
        Self::new(location, NodeDataTypeInternal::Document(DocumentData::new(quirks_mode)))
    }

    #[must_use]
    pub fn new_doctype(location: Location, name: &str, pub_identifier: &str, sys_identifier: &str) -> Self {
        Self::new(
            location,
            NodeDataTypeInternal::DocType(DocTypeData::new(name, pub_identifier, sys_identifier)),
        )
    }

    /// Create a new element node with the given name and attributes and namespace
    #[must_use]
    pub fn new_element(
        location: Location,
        name: &str,
        namespace: Option<&str>,
        attributes: HashMap<String, String>,
    ) -> Self {
        Self::new(
            location,
            NodeDataTypeInternal::Element(ElementData::new(name, namespace, attributes, Default::default())),
        )
    }

    /// Creates a new comment node
    #[must_use]
    pub fn new_comment(location: Location, value: &str) -> Self {
        Self::new(location, NodeDataTypeInternal::Comment(CommentData::with_value(value)))
    }

    /// Creates a new text node
    #[must_use]
    pub fn new_text(location: Location, value: &str) -> Self {
        Self::new(location, NodeDataTypeInternal::Text(TextData::with_value(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::fragment::DocumentFragmentImpl;
    use crate::node::elements::SPECIAL_HTML_ELEMENTS;
    use crate::node::elements::SPECIAL_MATHML_ELEMENTS;
    use crate::node::elements::SPECIAL_SVG_ELEMENTS;
    use crate::node::HTML_NAMESPACE;
    use crate::node::MATHML_NAMESPACE;
    use crate::node::SVG_NAMESPACE;
    use crate::DocumentBuilderImpl;
    use gosub_css3::system::Css3System;
    use gosub_interface::config::HasCssSystem;
    // use gosub_interface::document::DocumentBuilder;
    use gosub_interface::node::ElementDataType;
    use std::collections::HashMap;

    #[derive(Clone, Debug, PartialEq)]
    struct Config;

    impl HasCssSystem for Config {
        type CssSystem = Css3System;
    }
    impl HasDocument for Config {
        type Document = DocumentImpl<Self>;
        type DocumentFragment = DocumentFragmentImpl<Self>;
        type DocumentBuilder = DocumentBuilderImpl;
    }

    #[test]
    fn new_document() {
        let node = NodeImpl::<Config>::new_document(Location::default(), QuirksMode::NoQuirks);
        assert_eq!(node.id, NodeId::default());
        assert_eq!(node.parent, None);
        assert!(node.children.is_empty());
        if let NodeDataTypeInternal::Document(_) = &node.data {
        } else {
            panic!()
        }
    }

    #[test]
    fn new_element() {
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "test".to_string());

        let node = NodeImpl::<Config>::new_element(Location::default(), "div", Some(HTML_NAMESPACE), attributes);
        assert_eq!(node.id, NodeId::default());
        assert_eq!(node.parent, None);
        assert!(node.children.is_empty());

        if let NodeDataTypeInternal::Element(data) = &node.data {
            assert_eq!(data.name(), "div");
            assert!(data.attributes().contains_key("id"));
            assert_eq!(data.attributes().get("id").unwrap(), "test");
        } else {
            panic!()
        }
    }

    #[test]
    fn new_comment() {
        let node = NodeImpl::<Config>::new_comment(Location::default(), "test");
        assert_eq!(node.id, NodeId::default());
        assert_eq!(node.parent, None);
        assert!(node.children.is_empty());
        let NodeDataTypeInternal::Comment(CommentData { value, .. }) = &node.data else {
            panic!()
        };
        assert_eq!(value, "test");
    }

    #[test]
    fn new_text() {
        let node = NodeImpl::<Config>::new_text(Location::default(), "test");
        assert_eq!(node.id, NodeId::default());
        assert_eq!(node.parent, None);
        assert!(node.children.is_empty());
        let NodeDataTypeInternal::Text(TextData { value }) = &node.data else {
            panic!()
        };
        assert_eq!(value, "test");
    }

    #[test]
    fn is_special() {
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "test".to_string());

        let node = NodeImpl::<Config>::new_element(Location::default(), "div", Some(HTML_NAMESPACE), attributes);
        assert!(node.get_element_data().unwrap().is_special());
    }

    #[test]
    fn type_of() {
        let node = NodeImpl::<Config>::new_document(Location::default(), QuirksMode::NoQuirks);
        assert_eq!(node.type_of(), NodeType::DocumentNode);
        let node = NodeImpl::<Config>::new_text(Location::default(), "test");
        assert_eq!(node.type_of(), NodeType::TextNode);
        let node = NodeImpl::<Config>::new_comment(Location::default(), "test");
        assert_eq!(node.type_of(), NodeType::CommentNode);
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "test".to_string());
        let node = NodeImpl::<Config>::new_element(Location::default(), "div", Some(HTML_NAMESPACE), attributes);
        assert_eq!(node.type_of(), NodeType::ElementNode);
    }

    #[test]
    fn special_html_elements() {
        for element in &SPECIAL_HTML_ELEMENTS {
            let mut attributes = HashMap::new();
            attributes.insert("id".to_string(), "test".to_string());

            let node = NodeImpl::<Config>::new_element(Location::default(), element, Some(HTML_NAMESPACE), attributes);

            assert!(node.get_element_data().unwrap().is_special());
        }
    }

    #[test]
    fn special_mathml_elements() {
        for element in &SPECIAL_MATHML_ELEMENTS {
            let mut attributes = HashMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            let node = NodeImpl::<Config>::new_element(
                Location::default(),
                element,
                Some(MATHML_NAMESPACE),
                attributes.clone(),
            );

            assert!(node.get_element_data().unwrap().is_special());
        }
    }

    #[test]
    fn special_svg_elements() {
        for element in &SPECIAL_SVG_ELEMENTS {
            let mut attributes = HashMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            let node = NodeImpl::<Config>::new_element(Location::default(), element, Some(SVG_NAMESPACE), attributes);
            assert!(node.get_element_data().unwrap().is_special());
        }
    }

    #[test]
    fn type_of_node() {
        let node = NodeImpl::<Config>::new_document(Location::default(), QuirksMode::NoQuirks);
        assert_eq!(node.type_of(), NodeType::DocumentNode);
        let node = NodeImpl::<Config>::new_text(Location::default(), "test");
        assert_eq!(node.type_of(), NodeType::TextNode);
        let node = NodeImpl::<Config>::new_comment(Location::default(), "test");
        assert_eq!(node.type_of(), NodeType::CommentNode);
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "test".to_string());
        let node = NodeImpl::<Config>::new_element(Location::default(), "div", Some(HTML_NAMESPACE), attributes);
        assert_eq!(node.type_of(), NodeType::ElementNode);
    }
}
