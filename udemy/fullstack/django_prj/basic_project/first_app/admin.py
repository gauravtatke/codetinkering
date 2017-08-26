from django.contrib import admin
from first_app.models import AccessRecords, Topic, Webpage, Users

# Register your models here.
admin.site.register(AccessRecords)
admin.site.register(Topic)
admin.site.register(Webpage)
admin.site.register(Users)
