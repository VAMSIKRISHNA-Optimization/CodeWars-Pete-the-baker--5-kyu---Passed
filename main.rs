use std::collections::HashMap;
fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 
{
    if recipe.keys().all(|k| available.contains_key(k))
    {
            recipe.iter()
            .map(|(key, &rec_val)| 
                {
                let &avail_val = available.get(key).unwrap();
                avail_val / rec_val 
                })
                .min() 
                .unwrap_or(0)
    }
    else
    {
        return 0;
    }
