using System;

namespace TestNS
{
	class TestClass
	{ 
		String TestField = "Test";

		String getTestField()
		{
			return "Call result: " + this.TestField;
		}
	}
}
