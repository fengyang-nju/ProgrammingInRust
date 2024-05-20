fn main() {
    {
        // In the classroom,
        // the one callled "marisa" is a teacher.
        let marisa = "teacher";
        assert_eq!(marisa, "teacher");
    }
    // assert_eq!(marisa, "teacher");
    // Not ok; marisa is not valid here.
    {
        // In the dormitory,
        // The one callled "marisa" is my roommate
        let marisa = "roommate";
        assert_eq!(marisa, "roommate");
    }
}
