using System;

class Person
{
    public string firstName;
    public string lastName;

    public Person(string firstName, string lastName)
    {
        this.firstName = firstName;
        this.lastName = lastName;
    }
}

class Main
{
    static void Main(string[] args)
    {
        int age = 24;

        Person person1 = new Person("John", "Doe");
        Person person2 = person1;
        person2.firstName = "Jane";
    }
}