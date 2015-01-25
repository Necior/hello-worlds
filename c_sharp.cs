namespace HelloApp {
	class HelloClass {
		static void Main(string[] args) {
			System.Console.WriteLine("Hello, what's your name?");
			System.Console.Write("> ");
			string name = System.Console.ReadLine();
			System.Console.WriteLine("Nice to meet you, " + name + ".");
		}
	}
}
