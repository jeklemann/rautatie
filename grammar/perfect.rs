use crate::grammar::participles::*;
use crate::grammar::transforms::*;
use crate::verb::Verb;

use super::Tense;

pub struct PerfectTense {
    base: &'static dyn Tense,
}

impl Tense for PerfectTense {
    fn first_person_singular_positive(&self, verb: &mut Verb) {
        past_active_participle(verb, false);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.first_person_singular_positive(verb);
            };

            return prepend_olla_form(verb, "present first person singular positive", &form_func);
        });
    }

    fn second_person_singular_positive(&self, verb: &mut Verb) {
        past_active_participle(verb, false);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.second_person_singular_positive(verb);
            };

            return prepend_olla_form(verb, "present second person singular positive", &form_func);
        });
    }

    fn third_person_singular_positive(&self, verb: &mut Verb) {
        past_active_participle(verb, false);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.third_person_singular_positive(verb);
            };

            return prepend_olla_form(verb, "present third person singular positive", &form_func);
        });
    }

    fn first_person_plural_positive(&self, verb: &mut Verb) {
        past_active_participle(verb, true);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.first_person_plural_positive(verb);
            };

            return prepend_olla_form(verb, "present first person plural positive", &form_func);
        });
    }

    fn second_person_plural_positive(&self, verb: &mut Verb) {
        past_active_participle(verb, true);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.second_person_plural_positive(verb);
            };

            return prepend_olla_form(verb, "present second person plural positive", &form_func);
        });
    }

    fn third_person_plural_positive(&self, verb: &mut Verb) {
        past_active_participle(verb, true);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.third_person_plural_positive(verb);
            };

            return prepend_olla_form(verb, "present third person plural positive", &form_func);
        });
    }

    fn passive_positive(&self, verb: &mut Verb) {
        past_passive_participle(verb);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.passive_positive(verb);
            };

            return prepend_olla_form(verb, "present third person singular positive", &form_func);
        });
    }

    fn first_person_singular_negative(&self, verb: &mut Verb) {
        past_active_participle(verb, false);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.first_person_singular_negative(verb);
            };

            return prepend_olla_form(verb, "present first person singular negative", &form_func);
        });
    }

    fn second_person_singular_negative(&self, verb: &mut Verb) {
        past_active_participle(verb, false);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.second_person_singular_negative(verb);
            };

            return prepend_olla_form(verb, "present second person singular negative", &form_func);
        });
    }

    fn third_person_singular_negative(&self, verb: &mut Verb) {
        past_active_participle(verb, false);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.third_person_singular_negative(verb);
            };

            return prepend_olla_form(verb, "present third person singular negative", &form_func);
        });
    }

    fn first_person_plural_negative(&self, verb: &mut Verb) {
        past_active_participle(verb, true);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.first_person_plural_negative(verb);
            };

            return prepend_olla_form(verb, "present first person plural negative", &form_func);
        });
    }

    fn second_person_plural_negative(&self, verb: &mut Verb) {
        past_active_participle(verb, true);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.second_person_plural_negative(verb);
            };

            return prepend_olla_form(verb, "present second person plural negative", &form_func);
        });
    }

    fn third_person_plural_negative(&self, verb: &mut Verb) {
        past_active_participle(verb, true);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.third_person_plural_negative(verb);
            };

            return prepend_olla_form(verb, "present third person plural negative", &form_func);
        });
    }

    fn passive_negative(&self, verb: &mut Verb) {
        past_passive_participle(verb);

        verb.transform(&|verb| {
            let form_func = |verb: &mut Verb| {
                self.base.passive_negative(verb);
            };

            return prepend_olla_form(verb, "present third person singular negative", &form_func);
        });
    }
}

pub const fn create_perfect_from_base(tense: &'static dyn Tense) -> PerfectTense {
    return PerfectTense { base: tense };
}
