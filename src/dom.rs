use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let text_node = text("hello".to_string());
        assert_eq!(text_node.node_type, NodeType::Text("hello".to_string()));
    }

    #[test]
    fn test_elem() {
        let elem_node = elem(
            "div".to_string(),
            HashMap::new(),
            vec![text("hello".to_string())],
        );
        assert_eq!(
            elem_node.node_type,
            NodeType::Element(ElementData {
                tag_name: "div".to_string(),
                attributes: HashMap::new(),
            })
        );
    }

    #[test]
    fn test_elem_with_attrs() {
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "foo".to_string());
        attrs.insert("id".to_string(), "bar".to_string());
        let elem_node = elem("div".to_string(), attrs, vec![]);
        assert_eq!(
            elem_node.node_type,
            NodeType::Element(ElementData {
                tag_name: "div".to_string(),
                attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("class".to_string(), "foo".to_string());
                    attrs.insert("id".to_string(), "bar".to_string());
                    attrs
                },
            })
        );
    }


}
