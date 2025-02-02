use std::collections::HashMap;
use secp256k1::serde::de::Unexpected::Str;
use crate::cutils::{clone_vec, clone_vvec};
use crate::lib::custom_types::{VString, VVString};

#[derive(Clone)]
pub struct PremInfo
{
    count: u32,
    variety: VString,
}


pub struct PermutationHandler {
    pub(crate) m_elements: VString,
    pub(crate) m_shouldBeUnique: bool,
    pub(crate) m_permutations: VVString,
    pub(crate) m_permutationsStringify: VString,
    pub(crate) m_test_analyze: HashMap<String, PremInfo>,
}

impl PermutationHandler {
    pub(crate) fn new(
        elements: &VString,
        subset_count: u32,
        shouldBeUnique: bool,
        premutions: &VVString,
        premutionsStringify: &VString,
    ) -> PermutationHandler {
        let test_analyze: HashMap<String, PremInfo> = HashMap::new();
        let mut o: PermutationHandler = PermutationHandler {
            m_elements: clone_vec(elements),
            m_shouldBeUnique: shouldBeUnique,
            m_permutations: clone_vvec(premutions),
            m_permutationsStringify: clone_vec(premutionsStringify),
            m_test_analyze: test_analyze,
        };
        if subset_count > 0 {
            o.recursiveHeapP(&o.m_elements.clone(), subset_count, -1);
        }

        return o;
    }

    pub(crate) fn testAnalyze(&mut self, premutations_: &VVString) {
        let mut premutations: VVString = clone_vvec(premutations_);
        if premutations.len() == 0
        {
            premutations = clone_vvec(&self.m_permutations);
        }

        for mut a_set in premutations {
            let an_not_ordered_key: String = a_set.join(",");
            a_set.sort();
            let an_ordered_key: String = a_set.join(",");
            println!("an_ordered_key {}", an_ordered_key);
            let mut a_prem_info: PremInfo;
            if self.m_test_analyze.contains_key(&an_ordered_key) {
                a_prem_info = self.m_test_analyze.get(&an_ordered_key).unwrap().clone();
            } else {
                a_prem_info = PremInfo { count: 0, variety: vec![] };
            }

            if a_prem_info.count == 0 {
                self.m_test_analyze.insert(an_ordered_key.clone(), a_prem_info.clone());
            }
            a_prem_info.count += 1;
            a_prem_info.variety.push(an_not_ordered_key);
            self.m_test_analyze.insert(an_ordered_key.clone(), a_prem_info.clone());
        }
    }

    pub(crate) fn recursiveHeapP(&mut self,
                                 values: &VString,
                                 subset_count: u32,
                                 inner_size: i32)
    {

        // FIXME: implement automatic calculation START
        let mod_: String = format!("{}/{}", subset_count, values.len());

        if mod_ == "1/1" {
            self.m_permutations = vec![vec![values[0].clone()]];
            return;
        }
        if mod_ == "1/2" {
            self.m_permutations = vec![
                vec![values[0].clone()],
                vec![values[1].clone()],
            ];
            return;
        }

        if mod_ == "2/2" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone()]
            ];
            return;
        }

        if mod_ == "1/3" {
            self.m_permutations = vec![
                vec![values[0].clone()],
                vec![values[1].clone()],
                vec![values[2].clone()],
            ];
            return;
        }

        if mod_ == "2/3" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone()],
                vec![values[0].clone(), values[2].clone()],
                vec![values[1].clone(), values[2].clone()],
            ];
            return;
        }

        if mod_ == "3/3" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone(), values[2].clone()]
            ];
            return;
        }

        if mod_ == "1/4" {
            self.m_permutations = vec![
                vec![values[0].clone()],
                vec![values[1].clone()],
                vec![values[2].clone()],
                vec![values[3].clone()],
            ];
            return;
        }

        if mod_ == "2/4" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone()],
                vec![values[0].clone(), values[2].clone()],
                vec![values[0].clone(), values[3].clone()],
                vec![values[1].clone(), values[2].clone()],
                vec![values[1].clone(), values[3].clone()],
                vec![values[2].clone(), values[3].clone()],
            ];
            return;
        }

        if mod_ == "3/4" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone(), values[2].clone()],
                vec![values[0].clone(), values[1].clone(), values[3].clone()],
                vec![values[0].clone(), values[2].clone(), values[3].clone()],
                vec![values[1].clone(), values[2].clone(), values[3].clone()],
            ];
            return;
        }

        if mod_ == "4/4" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[3].clone()]
            ];
            return;
        }

        if mod_ == "1/5" {
            self.m_permutations = vec![
                vec![values[0].clone()],
                vec![values[1].clone()],
                vec![values[2].clone()],
                vec![values[3].clone()],
                vec![values[4].clone()],
            ];
            return;
        }

        if mod_ == "2/5" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone()],
                vec![values[0].clone(), values[2].clone()],
                vec![values[0].clone(), values[3].clone()],
                vec![values[0].clone(), values[4].clone()],
                vec![values[1].clone(), values[2].clone()],
                vec![values[1].clone(), values[3].clone()],
                vec![values[1].clone(), values[4].clone()],
                vec![values[2].clone(), values[3].clone()],
                vec![values[2].clone(), values[4].clone()],
                vec![values[3].clone(), values[4].clone()],
            ];
            return;
        }

        if mod_ == "3/5" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone(), values[2].clone()],
                vec![values[0].clone(), values[1].clone(), values[3].clone()],
                vec![values[0].clone(), values[1].clone(), values[4].clone()],
                vec![values[0].clone(), values[2].clone(), values[3].clone()],
                vec![values[0].clone(), values[2].clone(), values[4].clone()],
                vec![values[0].clone(), values[3].clone(), values[4].clone()],
                vec![values[1].clone(), values[2].clone(), values[3].clone()],
                vec![values[1].clone(), values[2].clone(), values[4].clone()],
                vec![values[1].clone(), values[3].clone(), values[4].clone()],
                vec![values[2].clone(), values[3].clone(), values[4].clone()],
            ];
            return;
        }

        if mod_ == "4/5" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[3].clone()],
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[4].clone()],
                vec![values[0].clone(), values[1].clone(), values[3].clone(), values[4].clone()],
                vec![values[0].clone(), values[2].clone(), values[3].clone(), values[4].clone()],
                vec![values[1].clone(), values[2].clone(), values[3].clone(), values[4].clone()],
            ];
            return;
        }

        if mod_ == "5/5" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[3].clone(), values[4].clone()]
            ];
            return;
        }

        // it is a custom 5/7. later implement a homogenous premutation
        if mod_ == "5/7" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[3].clone(), values[4].clone()],  // 5,6
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[3].clone(), values[5].clone()],  // 4,6
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[4].clone(), values[5].clone()],  // 3,6
                vec![values[0].clone(), values[1].clone(), values[3].clone(), values[4].clone(), values[5].clone()],  // 2,6
                vec![values[0].clone(), values[2].clone(), values[3].clone(), values[4].clone(), values[5].clone()],  // 1,6
                vec![values[1].clone(), values[2].clone(), values[3].clone(), values[4].clone(), values[5].clone()],  // 0,6
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[3].clone(), values[6].clone()],  // 4,5
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[4].clone(), values[6].clone()],  // 3,5
                vec![values[0].clone(), values[1].clone(), values[3].clone(), values[4].clone(), values[6].clone()],  // 2,5
                vec![values[0].clone(), values[2].clone(), values[3].clone(), values[4].clone(), values[6].clone()],  // 1,5
                vec![values[1].clone(), values[2].clone(), values[3].clone(), values[4].clone(), values[6].clone()],  // 0,5
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[5].clone(), values[6].clone()],  // 3,4
                vec![values[0].clone(), values[1].clone(), values[3].clone(), values[5].clone(), values[6].clone()],  // 2,4
                vec![values[0].clone(), values[2].clone(), values[3].clone(), values[5].clone(), values[6].clone()],  // 1,4
                vec![values[1].clone(), values[2].clone(), values[3].clone(), values[5].clone(), values[6].clone()],  // 0,4
                vec![values[0].clone(), values[1].clone(), values[4].clone(), values[5].clone(), values[6].clone()],  // 2,3
                vec![values[0].clone(), values[2].clone(), values[4].clone(), values[5].clone(), values[6].clone()],  // 1,3
                vec![values[1].clone(), values[2].clone(), values[4].clone(), values[5].clone(), values[6].clone()],  // 0,3
                vec![values[0].clone(), values[3].clone(), values[4].clone(), values[5].clone(), values[6].clone()],  // 1,2
                vec![values[1].clone(), values[3].clone(), values[4].clone(), values[5].clone(), values[6].clone()],  // 0,2
                vec![values[2].clone(), values[3].clone(), values[4].clone(), values[5].clone(), values[6].clone()],  // 0,1
            ];
            return;
        }

        if mod_ == "6/7" {
            self.m_permutations = vec![
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[3].clone(), values[4].clone(), values[5].clone()],
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[3].clone(), values[4].clone(), values[6].clone()],
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[3].clone(), values[5].clone(), values[6].clone()],
                vec![values[0].clone(), values[1].clone(), values[2].clone(), values[4].clone(), values[5].clone(), values[6].clone()],
                vec![values[0].clone(), values[1].clone(), values[3].clone(), values[4].clone(), values[5].clone(), values[6].clone()],
                vec![values[0].clone(), values[2].clone(), values[3].clone(), values[4].clone(), values[5].clone(), values[6].clone()],
                vec![values[1].clone(), values[2].clone(), values[3].clone(), values[4].clone(), values[5].clone(), values[6].clone()],
            ];
            return;
        }
    }
}