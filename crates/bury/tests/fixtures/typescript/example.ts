// Example TypeScript code with dead code

function usedFunction(): number {
    // This function is called from main
    return 42;
}

function deadFunction(): string {
    // This function is never called - DEAD CODE
    return "I'm dead";
}

const anotherDeadFunction = () => {
    // Another unused function - DEAD CODE
    console.log("Nobody calls me");
};

class UsedClass {
    usedMethod(): string {
        // This method is called
        return "I'm alive";
    }

    deadMethod(): string {
        // This method is never called - DEAD CODE
        return "I'm dead too";
    }
}

function main() {
    // Entry point
    const result = usedFunction();
    const obj = new UsedClass();
    obj.usedMethod();
    console.log(`Result: ${result}`);
}

main();
