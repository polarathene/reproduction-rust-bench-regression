use std::collections::HashMap;

pub fn fold(data: &Map) -> String {
    let s = String::with_capacity(data.count_bytes());

    data.iter().map(|(_id, params)| {
        // Replacing this with the methods body will avoid the regression
        kv_pairs_iter(params)
    })
    .fold(s, |mut s, kv_pairs| {
        // Removing either of these lines will avoid the regression
        // For some reason the push is necessary to reproduce
        s.push('[');
        s.extend(kv_pairs);

        s
    })
}

// Just a copy of the above function is sufficient to add a perf regression
// Remove it to avoid the regression, the `fastest` timing will otherwise increase by 20-30ns
pub fn fold_copy(data: &Map) -> String {
    let s = String::with_capacity(data.count_bytes());

    data.iter().map(|(_id, params)| {
        // Replacing this with the methods body will avoid the regression
        kv_pairs_iter(params)
    })
    .fold(s, |mut s, kv_pairs| {
        s.push('['); // Removing this line has no improvement
        s.extend(kv_pairs); // Removing this line avoids the regression

        s
    })
}

// Duplicating this iterator instead of being DRY via the method call resolves the regression
// Using #[inline(always)] here makes no difference
fn kv_pairs_iter(params: &HashMap<String, String>) -> impl Iterator<Item=&str> {
    let kv_pairs = params.iter().flat_map(|(key, value)| {
        [" ", key, "=\"", value, "\""]
    });

    kv_pairs
}


// Input type and mock data prep below, should not be relevant to regression?

pub struct Map(HashMap<String, HashMap<String, String>>);

impl Map {
    pub fn new() -> Self {
        let inner = FakeData::default().gen_map();
        Self(inner)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &HashMap<String, String>)> {
        self.0.iter()
    }

    pub fn count_bytes(&self) -> usize {
        &self.0.iter().map(|(id, params)| {
            params.iter().fold(id.len() + (params.len() * 4), |len, (key, value)| {
                len + key.len() + value.len()
            })
        }).sum::<usize>() + (&self.0.len() * 2)
    }
}

#[derive(Default)]
pub struct FakeData {
    rng: fastrand::Rng,
}

impl FakeData {
    fn gen_map(&mut self) -> HashMap<String, HashMap<String, String>> {
        (0..4).map(|_| (self.gen_string(8), self.gen_params(3))).collect()
    }
    
    fn gen_params(&mut self, num_pairs: usize) -> HashMap<String, String> {
        (0..num_pairs).map(|_| (self.gen_string(8), self.gen_string(12))).collect()
    }
    
    fn gen_string(&mut self, char_count: usize) -> String {
        (0..char_count).map(|_| self.rng.alphanumeric()).collect()
    }
}
