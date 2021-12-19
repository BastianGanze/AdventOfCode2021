use crate::Solution;
use std::cmp::max;
use std::collections::HashMap;

pub type SnailFishNodeId = u32;

#[derive(Debug, Clone)]
pub struct SnailFishNumber {
    main_node: SnailFishNodeId,
    max_depth: u32,
    nodes: HashMap<SnailFishNodeId, SnailFishNumberNode>,
}

impl SnailFishNumber {
    pub fn from_string(
        snail_fish_number_string: &str,
        id_gen: &mut SnailFishNodeId,
    ) -> SnailFishNumber {
        let mut snail_fish_number = SnailFishNumber {
            main_node: 0,
            max_depth: 0,
            nodes: HashMap::new(),
        };

        snail_fish_number.main_node = snail_fish_number.add_node(id_gen);

        let mut chars = snail_fish_number_string.chars().into_iter().peekable();

        let mut current_node = snail_fish_number.main_node;
        let mut current_depth = 0;
        let mut current_number_left: Option<SnailFishNodeId> = None;
        let mut last_was_comma = false;
        let mut max_depth = 0;

        while let Some(char) = chars.next() {
            match char {
                ',' => {
                    last_was_comma = true;
                }
                ']' => {
                    if let Some(parent) = snail_fish_number.get_parent(&current_node) {
                        current_node = parent;
                    }
                    current_depth -= 1;
                }
                '[' => {
                    let new_node = snail_fish_number.add_node(id_gen);

                    if last_was_comma {
                        last_was_comma = false;
                        snail_fish_number.set_child_right(&current_node, &new_node);
                    } else {
                        snail_fish_number.set_child_left(&current_node, &new_node);
                    }

                    snail_fish_number.set_parent(&new_node, &current_node);
                    snail_fish_number.set_depth(&new_node, current_depth);

                    current_node = new_node;
                    current_depth += 1;
                    max_depth = max(max_depth, current_depth);
                }
                num_as_char => {
                    let num: u32 = num_as_char.to_digit(10).unwrap();
                    let new_literal = snail_fish_number.add_node(id_gen);

                    if last_was_comma {
                        last_was_comma = false;
                        snail_fish_number.set_child_right(&current_node, &new_literal);
                    } else {
                        snail_fish_number.set_child_left(&current_node, &new_literal);
                    }

                    snail_fish_number.set_parent(&new_literal, &current_node);
                    snail_fish_number.set_depth(&new_literal, current_depth);

                    snail_fish_number.set_node_type(&new_literal, NodeType::Number(num));

                    if let Some(current_left) = current_number_left {
                        snail_fish_number.set_right_number(&current_left, &new_literal);
                        snail_fish_number.set_left_number(&new_literal, &current_left);
                    }

                    current_number_left = Some(new_literal);
                }
            }
        }

        let actual_main_node = snail_fish_number
            .nodes
            .get_mut(
                &snail_fish_number
                    .get_child_left(&snail_fish_number.main_node)
                    .unwrap(),
            )
            .unwrap();
        actual_main_node.parent = None;
        snail_fish_number.main_node = actual_main_node.id;

        snail_fish_number.max_depth = max_depth;

        snail_fish_number
    }

    pub fn add(&mut self, snail_fish_number: &SnailFishNumber, id_gen: &mut SnailFishNodeId) {
        let own_most_right_number = &self.most_right_number(&self.main_node);
        let other_most_left_number =
            &snail_fish_number.most_left_number(&snail_fish_number.main_node);

        for (key, value) in snail_fish_number.nodes.iter() {
            self.nodes.insert(key.clone(), value.clone());
        }

        let new_parent_node = self.add_node(id_gen);

        let left = self.main_node;
        let right = snail_fish_number.main_node;

        self.set_parent(&left, &new_parent_node);
        self.set_parent(&right, &new_parent_node);

        self.set_child_left(&new_parent_node, &left);
        self.set_child_right(&new_parent_node, &right);

        self.main_node = new_parent_node;

        self.set_right_number(own_most_right_number, other_most_left_number);
        self.set_left_number(other_most_left_number, own_most_right_number);

        self._inc_depth();

        self.reduce(id_gen);
    }

    fn _inc_depth(&mut self) {
        for val in self.nodes.values_mut() {
            val.depth += 1;
        }
        self.nodes.get_mut(&self.main_node).unwrap().depth -= 1;
    }

    pub fn most_left_number(&self, node: &SnailFishNodeId) -> SnailFishNodeId {
        if let NodeType::Number(_) = self.get_node_type(node) {
            return node.clone();
        }

        self.most_left_number(&self.get_child_left(node).unwrap())
    }

    pub fn most_right_number(&self, node: &SnailFishNodeId) -> SnailFishNodeId {
        if let NodeType::Number(_) = self.get_node_type(node) {
            return node.clone();
        }

        self.most_right_number(&self.get_child_right(node).unwrap())
    }

    pub fn to_string(&self) -> String {
        self._to_string(&self.main_node)
    }

    fn _to_string(&self, node_id: &SnailFishNodeId) -> String {
        if let NodeType::Number(num) = self.get_node_type(node_id) {
            return num.to_string();
        }
        let mut str = String::new();
        let left = self.get_child_left(node_id).unwrap();
        let right = self.get_child_right(node_id).unwrap();
        str.push_str("[");
        str.push_str(self._to_string(&left).as_str());
        str.push_str(",");
        str.push_str(self._to_string(&right).as_str());
        str.push_str("]");
        str
    }

    fn reduce(&mut self, id_gen: &mut SnailFishNodeId) {
        loop {
            while let Some(explosion_id) = self.get_next_explosion() {
                self.explode(&explosion_id);
            }

            if let Some(split_node) = self.get_next_split() {
                self.split(&split_node, id_gen);
            } else {
                break;
            }
        }
    }

    fn get_next_explosion(&self) -> Option<SnailFishNodeId> {
        let mut current = self.most_left_number(&self.main_node);

        loop {
            let parent = &self.get_parent(&current).unwrap();
            let (left, right) = (
                self.get_node_type(&self.get_child_left(parent).unwrap()),
                self.get_node_type(&self.get_child_right(parent).unwrap()),
            );
            if left != NodeType::Node && right != NodeType::Node && self.get_depth(&parent) >= 4 {
                return Some(parent.clone());
            }

            if let Some(next_number) = self.get_right_number(&current) {
                current = next_number;
            } else {
                break;
            }
        }
        None
    }

    fn get_next_split(&self) -> Option<SnailFishNodeId> {
        let mut current = self.most_left_number(&self.main_node);
        if self.get_literal_value(&current) >= 10 {
            return Some(current.clone());
        }
        while let Some(next_number) = self.get_right_number(&current) {
            current = next_number;
            if self.get_literal_value(&current) >= 10 {
                return Some(current.clone());
            }
        }
        None
    }

    fn explode(&mut self, node_id: &SnailFishNodeId) {
        let current_left = self.get_child_left(node_id).unwrap();
        let current_right = self.get_child_right(node_id).unwrap();
        let number_left_of_left_o = self.get_left_number(&current_left);
        let number_right_of_right_o = self.get_right_number(&current_right);

        self.explode_into(&current_left, number_left_of_left_o);
        self.explode_into(&current_right, number_right_of_right_o);

        self.node_into_literal(node_id, &current_left, &current_right);
        self.set_node_type(node_id, NodeType::Number(0));
        self.connect_left_and_right(node_id, number_left_of_left_o, number_right_of_right_o);

        let new_depth = self.get_depth(&node_id) - 1;
        self.set_depth(&node_id, new_depth);
    }

    fn connect_left_and_right(
        &mut self,
        node_id: &SnailFishNodeId,
        number_left_of_left_o: Option<SnailFishNodeId>,
        number_right_of_right_o: Option<SnailFishNodeId>,
    ) {
        if let Some(num) = number_left_of_left_o {
            self.set_left_number(node_id, &num);
            self.set_right_number(&num, node_id);
        }
        if let Some(num) = number_right_of_right_o {
            self.set_right_number(node_id, &num);
            self.set_left_number(&num, node_id);
        }
    }

    fn explode_into(&mut self, current: &SnailFishNodeId, other_number_o: Option<SnailFishNodeId>) {
        if let Some(other_number) = other_number_o {
            if let NodeType::Number(current_val) = self.get_node_type(&current) {
                if let NodeType::Number(other_number_val) = self.get_node_type(&other_number) {
                    self.set_node_type(
                        &other_number,
                        NodeType::Number(current_val + other_number_val),
                    );
                }
            }
        }
    }

    fn get_literal_value(&self, node_id: &SnailFishNodeId) -> u32 {
        if let NodeType::Number(val) = self.get_node_type(node_id) {
            return val;
        }
        unreachable!();
    }

    fn split(&mut self, node_id: &SnailFishNodeId, id_gen: &mut SnailFishNodeId) {
        let current_value = self.get_literal_value(node_id);

        let new_left = &self.add_node(id_gen);
        self.set_parent(new_left, node_id);
        self.set_node_type(
            new_left,
            NodeType::Number(((current_value as f64) / 2_f64).floor() as u32),
        );

        let new_right = &self.add_node(id_gen);
        self.set_parent(new_right, node_id);
        self.set_node_type(
            new_right,
            NodeType::Number(((current_value as f64) / 2_f64).ceil() as u32),
        );

        self.set_child_left(node_id, new_left);
        self.set_child_right(node_id, new_right);

        let old_left_num_o = self.get_left_number(node_id);
        let old_right_num_o = self.get_right_number(node_id);

        self.literal_into_node(node_id);

        self.set_right_number(new_left, new_right);
        self.set_left_number(new_right, new_left);

        if let Some(old_left_num) = &old_left_num_o {
            self.set_left_number(new_left, old_left_num);
            self.set_right_number(old_left_num, new_left);
        }

        if let Some(old_right_num) = &old_right_num_o {
            self.set_right_number(new_right, old_right_num);
            self.set_left_number(old_right_num, new_right);
        }

        self.set_depth(
            node_id,
            self.get_depth(&self.get_parent(node_id).unwrap()) + 1,
        );
    }

    pub fn magnitude(&mut self) -> Solution {
        self._magnitude(&self.main_node)
    }

    pub fn _magnitude(&self, node: &SnailFishNodeId) -> Solution {
        if let NodeType::Number(value) = self.get_node_type(node) {
            return value;
        }
        let left = &self.get_child_left(node).unwrap();
        let right = &self.get_child_right(node).unwrap();
        self._magnitude(left) * 3 + self._magnitude(right) * 2
    }

    fn add_node(&mut self, node_id_gen: &mut SnailFishNodeId) -> SnailFishNodeId {
        *node_id_gen += 1;
        self.nodes.insert(
            node_id_gen.clone(),
            SnailFishNumberNode::new(node_id_gen.clone()),
        );
        node_id_gen.clone()
    }

    fn set_parent(&mut self, node: &SnailFishNodeId, parent: &SnailFishNodeId) {
        self.nodes.get_mut(node).unwrap().parent = Some(parent.clone());
    }

    fn set_left_number(&mut self, node: &SnailFishNodeId, left_number: &SnailFishNodeId) {
        self.nodes.get_mut(node).unwrap().left_number = Some(left_number.clone());
    }

    fn set_right_number(&mut self, node: &SnailFishNodeId, right_number: &SnailFishNodeId) {
        self.nodes.get_mut(node).unwrap().right_number = Some(right_number.clone());
    }

    fn set_child_left(&mut self, node: &SnailFishNodeId, child_left: &SnailFishNodeId) {
        self.nodes.get_mut(node).unwrap().child_left = Some(child_left.clone());
    }

    fn set_child_right(&mut self, node: &SnailFishNodeId, child_right: &SnailFishNodeId) {
        self.nodes.get_mut(node).unwrap().child_right = Some(child_right.clone());
    }

    fn literal_into_node(&mut self, node_id: &SnailFishNodeId) {
        let node = self.nodes.get_mut(node_id).unwrap();
        node.left_number = None;
        node.right_number = None;
        node.node_type = NodeType::Node;
    }

    fn node_into_literal(
        &mut self,
        node_id: &SnailFishNodeId,
        left: &SnailFishNodeId,
        right: &SnailFishNodeId,
    ) {
        self.nodes.remove(left);
        self.nodes.remove(right);

        let node = self.nodes.get_mut(node_id).unwrap();
        node.child_left = None;
        node.child_right = None;
    }

    fn set_depth(&mut self, node: &SnailFishNodeId, depth: u32) {
        self.nodes.get_mut(node).unwrap().depth = depth;
    }

    fn set_node_type(&mut self, node: &SnailFishNodeId, node_type: NodeType) {
        self.nodes.get_mut(node).unwrap().node_type = node_type;
    }

    fn get_parent(&self, node: &SnailFishNodeId) -> Option<SnailFishNodeId> {
        self.nodes.get(node).unwrap().parent
    }

    fn get_left_number(&self, node: &SnailFishNodeId) -> Option<SnailFishNodeId> {
        self.nodes.get(node).unwrap().left_number
    }

    fn get_right_number(&self, node: &SnailFishNodeId) -> Option<SnailFishNodeId> {
        self.nodes.get(node).unwrap().right_number
    }

    fn get_child_left(&self, node: &SnailFishNodeId) -> Option<SnailFishNodeId> {
        self.nodes.get(node).unwrap().child_left
    }

    fn get_child_right(&self, node: &SnailFishNodeId) -> Option<SnailFishNodeId> {
        self.nodes.get(node).unwrap().child_right
    }

    fn get_depth(&self, node: &SnailFishNodeId) -> u32 {
        self.nodes.get(node).unwrap().depth
    }

    fn get_node_type(&self, node: &SnailFishNodeId) -> NodeType {
        self.nodes.get(node).unwrap().node_type
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeType {
    Number(u32),
    Node,
}

#[derive(Debug, Clone)]
pub struct SnailFishNumberNode {
    pub id: SnailFishNodeId,
    pub parent: Option<SnailFishNodeId>,
    pub left_number: Option<SnailFishNodeId>,
    pub right_number: Option<SnailFishNodeId>,
    pub child_left: Option<SnailFishNodeId>,
    pub child_right: Option<SnailFishNodeId>,
    pub node_type: NodeType,
    pub depth: u32,
}

impl SnailFishNumberNode {
    pub fn new(id: SnailFishNodeId) -> SnailFishNumberNode {
        SnailFishNumberNode {
            id,
            parent: None,
            child_left: None,
            child_right: None,
            left_number: None,
            node_type: NodeType::Node,
            right_number: None,
            depth: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::read_test;

    #[test]
    pub fn parse_snail_number() {
        let s1 = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".to_string();
        let mut ids = 0;
        let snail_fish_number = SnailFishNumber::from_string(&s1, &mut ids);
        assert_eq!(snail_fish_number.to_string(), s1);
    }

    #[test]
    pub fn add_snail_number_1() {
        let ss1 = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let ss2 = "[1,1]";
        let ss_result = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let mut ids = 0;
        let mut s1 = SnailFishNumber::from_string(ss1, &mut ids);
        let s2 = SnailFishNumber::from_string(ss2, &mut ids);
        let s_result = SnailFishNumber::from_string(ss_result, &mut ids);
        s1.add(&s2, &mut ids);
        assert_eq!(s1.to_string(), s_result.to_string());
    }

    #[test]
    pub fn add_snail_number_2() {
        let ss1 = "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]";
        let ss2 = "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]";
        let ss_result = "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]";
        let mut ids = 0;
        let mut s1 = SnailFishNumber::from_string(ss1, &mut ids);
        let s2 = SnailFishNumber::from_string(ss2, &mut ids);
        let s_result = SnailFishNumber::from_string(ss_result, &mut ids);
        s1.add(&s2, &mut ids);
        assert_eq!(s1.to_string(), s_result.to_string());
    }

    #[test]
    pub fn add_snail_number_3() {
        let ss1 = "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]";
        let ss2 = "[1,[[[9,3],9],[[9,0],[0,7]]]]";
        let ss_result = "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]";
        let mut ids = 0;
        let mut s1 = SnailFishNumber::from_string(ss1, &mut ids);
        let s2 = SnailFishNumber::from_string(ss2, &mut ids);
        let s_result = SnailFishNumber::from_string(ss_result, &mut ids);
        s1.add(&s2, &mut ids);
        assert_eq!(s1.to_string(), s_result.to_string());
    }

    #[test]
    pub fn add_snail_number_4() {
        let ss1 = "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]";
        let ss2 = "[[2,[2,2]],[8,[8,1]]]";
        let ss_result = "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]";
        let mut ids = 0;
        let mut s1 = SnailFishNumber::from_string(ss1, &mut ids);
        let s2 = SnailFishNumber::from_string(ss2, &mut ids);
        let s_result = SnailFishNumber::from_string(ss_result, &mut ids);
        s1.add(&s2, &mut ids);
        assert_eq!(s1.to_string(), s_result.to_string());
    }
}
