use yew::virtual_dom::VNode;

pub fn try_set_attributes_for_tag(node: &mut VNode, attributes: &[(&'static str, &'static str)]) -> Result<(), &'static str> {
    if let VNode::VTag(ref mut tag) = *node {
        let tag = std::rc::Rc::get_mut(&mut *tag).ok_or("Not a mutable (owned) instance")?;
        for attribute in attributes {
            tag.add_attribute(attribute.0, attribute.1);
        }

        Ok(())
    } else {
        Err("Unknown node type")
    }
}
