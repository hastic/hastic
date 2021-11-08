use super::types::ThresholdConfig;

struct ThresholdDetector {
    config: ThresholdConfig
}

impl ThresholdDetector {
    fn new(config: ThresholdConfig) -> ThresholdDetector {
        ThresholdDetector{ config }
    }

    pub fn detect(&self, ts: &Vec<(u64, f64)>) -> Vec<(u64, u64)> {

        let mut result = Vec::<(u64, u64)>::new();
        let mut from: Option<u64> = None;
        for (t, v) in ts {
            if *v > self.config.threashold {
                if from.is_some() {
                    continue;
                } else {
                    from = Some(*t);
                }
            } else {
                if from.is_some() {
                    result.push((from.unwrap(), *t));
                    from = None;
                }
            }
        }

        // TODO: don't repeat myself
        if from.is_some() {
            result.push((from.unwrap(), ts.last().unwrap().0));
        }

        // TODO: decide what to do it from is Some() in the end

        result
    }
}