class Cup:
    def __init__(self, value):
        self.value = value
        self.next: Cup = None
        self.lower: Cup = None

    def __repr__(self):
        return "cup({})".format(self.value)

    def __str__(self):
        return "cup({}),next:{},lower:{}".format(self.value, repr(self.next), repr(self.lower))

    def print_cups(self):
        print(self.value, end=" ")
        val = self.next
        while val != self:
            print(val.value, end=" ")
            val = val.next
        print()

    def get_cups_after_one(self):
        cup = self
        while cup.value != 1:
            cup = cup.next
        return cup.next.value * cup.next.next.value

    def get_labels(self):
        res = ""
        cup = self
        found_one = False
        while len(res) < 8:
            if not found_one and cup.value == 1:
                found_one = True
            elif found_one:
                res += str(cup.value)
            cup = cup.next
        return res

    @classmethod
    def get_cups(cls, input):
        first = Cup(int(input[0]))
        cup = first
        sorted_cups = {int(input[0]): cup}
        for char in input[1:]:
            cup.next = Cup(int(char))
            cup = cup.next
            sorted_cups[int(char)] = cup
        cup.next = first
        for val, cup in sorted_cups.items():
            cup.lower = sorted_cups[len(sorted_cups) if val == 1 else val - 1]
        return first

    @classmethod
    def get_many_cups(cls, input):
        first = Cup(int(input[0]))
        cup = first
        sorted_cups = {int(input[0]): cup}
        for char in input[1:]:
            cup.next = Cup(int(char))
            cup = cup.next
            sorted_cups[int(char)] = cup
        for i in range(10, 1_000_001):
            cup.next = Cup(i)
            cup = cup.next
            sorted_cups[i] = cup
        cup.next = first

        for val, cup in sorted_cups.items():
            cup.lower = sorted_cups[len(sorted_cups) if val == 1 else val - 1]
        return first


def do_move(current_cup: Cup):
    selected = [current_cup.next, current_cup.next.next, current_cup.next.next.next]
    current_cup.next = current_cup.next.next.next.next
    lower = current_cup.lower
    while lower in selected:
        lower = lower.lower
    after_lower = lower.next
    lower.next = selected[0]
    selected[2].next = after_lower


def do_moves(cup: Cup, num_moves):
    for i in range(num_moves):
        do_move(cup)
        cup = cup.next


def main():
    cup = Cup.get_cups("467528193")
    do_moves(cup, 100)
    cup.print_cups()
    print(cup.get_labels())

    cup = Cup.get_many_cups("467528193")
    do_moves(cup, 10_000_000)
    print(cup.get_cups_after_one())


if __name__ == '__main__':
    main()
