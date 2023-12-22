#[macro_export]
macro_rules! gen_tense_struct {
    () => {
        pub static TENSE_STRUCT: crate::grammar::Tense = crate::grammar::Tense {
            first_person_singular_positive: first_person_singular_positive,
            second_person_singular_positive: second_person_singular_positive,
            third_person_singular_positive: third_person_singular_positive,
            first_person_plural_positive: first_person_plural_positive,
            second_person_plural_positive: second_person_plural_positive,
            third_person_plural_positive: third_person_plural_positive,
            passive_positive: passive_positive,

            first_person_singular_negative: first_person_singular_negative,
            second_person_singular_negative: second_person_singular_negative,
            third_person_singular_negative: third_person_singular_negative,
            first_person_plural_negative: first_person_plural_negative,
            second_person_plural_negative: second_person_plural_negative,
            third_person_plural_negative: third_person_plural_negative,
            passive_negative: passive_negative,
        };
    };
}
