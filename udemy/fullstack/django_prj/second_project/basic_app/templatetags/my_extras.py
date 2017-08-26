from django import template

register = template.Library()


def cutval(value, arg):
    # removes arg from value string
    return value.replace(arg, '')


# first arg is name to reference in template, second arg is the function name
register.filter('callcut', cutval)
