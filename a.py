import re
from collections import Counter

def calculate_total_winnings(hands, card_order, joker_transform=False):
    def sort_key(hand):
        if joker_transform and 'J' in hand:
            highest_card = max((card for card in hand if card != 'J'), key=lambda c: card_order.index(c), default='2')
            hand = hand.replace('J', Counter(hand.replace('J', '')).most_common(1)[0][0] if hand.count('J') < 5 else highest_card)
        return (tuple(sorted(Counter(hand).values(), reverse=True)), tuple(card_order.index(c) for c in hand))

    return sum(w * (i + 1) for i, h in enumerate(sorted(hands.keys(), key=sort_key)) for w in [hands[h]])

def load_hands(file_path):
    with open(file_path, 'r') as file:
        return {h: int(b) for h, b in re.findall(r'(\w{5}) (\d+)', file.read())}


file_path = 'input.txt'
hands = load_hands(file_path)
print("Total Winnings Part 1:", calculate_total_winnings(hands, '23456789TJQKA'))
print("Total Winnings Part 2:", calculate_total_winnings(hands, 'J23456789TQKA', joker_transform=True))
