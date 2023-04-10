import timeit

from rustestes_lib import bindings

class RustLib:
    def __init__(self):
        self.lib = bindings.rustestest_lib()
    
    def reverse_listestest(self, input_list: list[int]) -> list[int]:
        return self.lib.reverse_listestest(x)
    
    def say_hi(self):
        print(self.lib.say_hi())

    def make_programmer(self, name: str, age: int, programming_languages: list[str]):
        print("\nSpawning new programmer...")
        programmer = self.lib.make_programmer(name, age, programming_languages)
        print(self.lib.introduce_programmer(programmer))
        print("\n")
        return programmer

    def high_computation(self, items: list[float]) -> list[float]:
        new_items = self.lib.high_computation(items)
        return new_items

def python_high_computation(items: list[float]) -> list[float]:
    maped_items = map(lambda x: (x ** 7.0) / 2.0, items)
    filtered_items = list(filter(lambda x: x > 10000.0, maped_items))
    return filtered_items
        

if __name__ == "__main__":
    lib = RustLib()
    lib.say_hi()

    programmer = lib.make_programmer(
        name="Jaw", 
        age=19, 
        programming_languages=[
            "Python", 
            "Rust"
        ]
    )

    print(programmer)

    x = [1, 2, 3]
    print(x)
    
    y = lib.reverse_listestest(x)
    print(y)

    listazinha = [z for z in range(1000)]
    print(lib.high_computation(listazinha)[0])
    print(python_high_computation(listazinha)[0])
    
    
    print(f"Python Computation: {timeit.timeit(lambda: python_high_computation(listazinha))}")
    print(f"Rust Computation: {timeit.timeit(lambda: lib.high_computation(listazinha))}")
