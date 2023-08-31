def get_max_product(values, factors = 0, product = 1):
    if factors == 3:
      return product

    max_product = None
    for value in values:
        other_values = values.copy()
        other_values.remove(value)

        current_product = get_max_product(other_values, factors + 1, value * product)
        if not max_product or current_product > max_product:
            max_product = current_product

    return max_product
