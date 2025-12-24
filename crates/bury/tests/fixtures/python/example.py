# Example Python code with dead code

def used_function():
    """This function is called from main"""
    return 42

def dead_function():
    """This function is never called - DEAD CODE"""
    return "I'm dead"

def another_dead_function():
    """Another unused function - DEAD CODE"""
    print("Nobody calls me")

class UsedClass:
    def used_method(self):
        """This method is called"""
        return "I'm alive"

    def dead_method(self):
        """This method is never called - DEAD CODE"""
        return "I'm dead too"

def main():
    """Entry point"""
    result = used_function()
    obj = UsedClass()
    obj.used_method()
    print(f"Result: {result}")

if __name__ == "__main__":
    main()
