//tm Test code
#[cfg(test)]
//a Imports
use stylesheet::{Descriptor, StylableNode, StyleTypeValue, Tree, TypeSet};

//a test_type_set
#[test]
fn test_type_set() {
    let int_type = StyleTypeValue::new(Option::<isize>::None);
    let style_set = TypeSet::default()
        .add_type("x", int_type.clone(), false)
        .add_type("y", int_type.clone(), false);
    assert_eq!(
        serde_json::to_string(&style_set).unwrap(),
        r###"{"set":{"x":{"type_value":{"Option<isize>":null},"inheritable":false},"y":{"type_value":{"Option<isize>":null},"inheritable":false}}}"###
    );
}

//a test_tree_rules
#[test]
fn test_tree_rules() {
    let int_type = StyleTypeValue::new(Option::<isize>::None);
    let style_set = TypeSet::default()
        .add_type("x", int_type.clone(), false)
        .add_type("y", int_type.clone(), false);
    let mut d_pt = Descriptor::new(&style_set);
    d_pt.add_style("x");
    d_pt.add_style("y");
    let d_g = Descriptor::new(&style_set);
    let mut node0_0 = StylableNode::new("pt", &d_pt);
    node0_0.add_name_value("x", "1").unwrap();
    node0_0.add_name_value("y", "0").unwrap();
    let mut node0_1 = StylableNode::new("pt", &d_pt);
    node0_1.add_name_value("x", "2").unwrap();
    node0_1.add_name_value("y", "10").unwrap();
    let mut group0 = StylableNode::new("g", &d_g);

    {
        let mut tree = Tree::new(&mut group0);
        tree.add_node(&mut node0_0);
        tree.add_node(&mut node0_1);
        tree.close_container();

        for top in tree.iter_tree() {
            top.as_option()
                .map(|(depth, n)| println!("{} {:?}", depth, n.borrow()));
        }
    }
}
