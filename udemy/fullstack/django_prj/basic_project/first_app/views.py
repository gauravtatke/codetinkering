from django.shortcuts import render
from django.http import HttpResponse
from first_app.models import Webpage, AccessRecords, Topic, Users
from first_app import forms
from first_app.forms import NewUserForm
# Create your views here.


def index(request):
    my_dict = {'insert_me': 'Hello! I am from templates fist_app!'}
    return render(request, 'first_app/index.html', context=my_dict)


def display(request):
    adict = {}
    return render(request, 'first_app/display.html', context=adict)


def table(request):
    wp_list = AccessRecords.objects.order_by('date')
    dt_dict = {'access_records': wp_list}
    return render(request, 'first_app/table.html', context=dt_dict)


def user(request):
    ulist = Users.objects.order_by('first_name')
    udict = {'urecords': ulist}
    return render(request, 'first_app/user.html', context=udict)


def form_view(request):
    form = forms.FormClass()
    if request.method == 'POST':
        form = forms.FormClass(request.POST)
        if form.is_valid():
            print('validation successful')
            print('name: ' + form.cleaned_data['name'])
            print('email: ' + form.cleaned_data['email'])
            print('text: ' + form.cleaned_data['text'])

    return render(request, 'first_app/form_page.html', context={'form': form})


def userregister(request):
    form = NewUserForm()
    if request.method == 'POST':
        form = NewUserForm(request.POST)

        if form.is_valid():
            form.save(commit=True)
            return index(request)
        else:
            print('ERROR! FORM IS NOT VALID')
    return render(request, 'first_app/userregister.html', context={'form': form})
