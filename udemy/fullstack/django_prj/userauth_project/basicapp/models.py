from django.db import models
from django.contrib.auth.models import User

# Create your models here.


class UserProfileInfo(models.Model):
    # don't inherit from User imported above
    # instead create one to one relationship with User
    # this will add fields in same User table which django admin already has
    user = models.OneToOneField(User)

    # additional fields
    portfolio_site = models.URLField(blank=True)
    profile_pic = models.ImageField(upload_to='profile_pics', blank=True)

    def __str__(self):
        return self.user.username
