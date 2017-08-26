import os
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'first_project.settings')

import django
django.setup()

# FAKE POPULATION OF DATABASES
from first_app.models import Users
from faker import Faker

fakeuser = Faker()


def populate_user(n=5):
    for i in range(n):
        fake_fname = fakeuser.first_name()
        fake_lname = fakeuser.last_name()
        fake_email = fakeuser.email()

        user = Users.objects.get_or_create(
            first_name=fake_fname, last_name=fake_lname, email=fake_email)


if __name__ == '__main__':
    print('populating user')
    populate_user(50)
    print('populating user complete')
