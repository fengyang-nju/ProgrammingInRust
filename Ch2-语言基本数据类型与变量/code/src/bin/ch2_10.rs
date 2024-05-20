fn main() {
    // In the classroom, your roommate sits beside you.
    // When talking with your roommate, "marisa" refers to your roomate.
    let marisa = "roommate";

    {
        assert_eq!(marisa, "roommate");
        // When asking questions to the teacher, "marisa" refers to the
        // teacher.

        let marisa = "teacher";
        assert_eq!(marisa, "teacher");
    }

    // After asking questions, you talk with your roomate, and "marisa"
    // refers to your roommate.
    assert_eq!(marisa, "roommate");

    // When your roommate leaves, "marisa" refers to the teacher.
    let marisa = "teacher";
    assert_eq!(marisa, "teacher");
}
