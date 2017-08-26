from django.db import models

# Create your models here.


class Topic(models.Model):
    top_name = models.CharField(max_length=128, unique=True)

    def __str__(self):
        return self.top_name


class Webpage(models.Model):
    topic = models.ForeignKey(Topic)
    name = models.CharField(max_length=128, unique=True)
    url = models.URLField(unique=True)

    def __str__(self):
        return self.name


class AccessRecords(models.Model):
    name = models.ForeignKey(Webpage)
    date = models.DateField()

    def __str__(self):
        return str(self.date)


class Users(models.Model):
    first_name = models.CharField(max_length=20)
    last_name = models.CharField(max_length=20)
    email = models.EmailField(unique=True)

    def __str__(self):
        return self.last_name + ', ' + self.first_name
