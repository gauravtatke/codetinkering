from django.shortcuts import render

# Create your views here.


def index(request):
    return render(request, 'basicapp/index.html')


def registration(request):
    required = False

    if request.method == 'POST':
        user_form = UserForm(data=request.POST)
        profile_form = UserProfileInfoForm(data=request.POST)

        if user_form.is_valid() and profile_form.is_valid():
