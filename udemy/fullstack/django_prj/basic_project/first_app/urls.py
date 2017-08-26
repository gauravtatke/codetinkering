from django.conf.urls import url
from first_app import views

urlpatterns = [
    url(r'^$', views.index, name='index'),
    url(r'^display', views.display, name='display'),
    url(r'^table', views.table, name='table'),
    url(r'^user$', views.user, name='user'),
    url(r'^form_page', views.form_view, name='formview'),
    url(r'^userregister', views.userregister, name='userregister'),
]
