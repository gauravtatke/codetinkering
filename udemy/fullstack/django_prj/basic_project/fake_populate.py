import os
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'first_project.settings')

import django
django.setup()

# FAKE POPULATION OF DATABASES
import random
from first_app.models import AccessRecords, Webpage, Topic
from faker import Faker

fakegen = Faker()
topics = ['Search', 'Social', 'Marketplace', 'News', 'Games']


def add_topic():
    t = Topic.objects.get_or_create(top_name=random.choice(topics))[0]
    t.save()
    return t


def populate(N=5):
    for entry in range(N):
        # get the topic for entry
        top = add_topic()

        # insert fake data
        fake_url = fakegen.url()
        fake_date = fakegen.date()
        fake_name = fakegen.company()

        # create a webpage entry
        wp = Webpage.objects.get_or_create(
            topic=top, name=fake_name, url=fake_url)[0]

        # create a AccessRecords entry
        ar = AccessRecords.objects.get_or_create(name=wp, date=fake_date)[0]


if __name__ == '__main__':
    print('populating records!')
    populate(20)
    print('complete!')
