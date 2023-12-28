import sys
import re
import pyperclip # type: ignore

def to_snake_case(text):
    snake_case = re.sub(r'[^a-zA-Z0-9]', '_', text)
    snake_case = snake_case.lower()
    snake_case = snake_case.lstrip('_')
    return snake_case

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 snake_case_converter.py <text>")
        sys.exit(1)

    input_text = sys.argv[1]
    snake_case_result = to_snake_case(input_text)

    pyperclip.copy(snake_case_result)

    print("Input text:", input_text)
    print("Snake case:", snake_case_result)
    print("Snake case copied to clipboard.")

if __name__ == "__main__":
    main()
