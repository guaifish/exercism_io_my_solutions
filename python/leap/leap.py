def leap_year(year):
    return not (year % 4 or (not year % 100 and year % 400))