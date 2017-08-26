from django import forms
from django.core import validators
from first_app.models import Users


def check_for_z(value):
    # parameter name has to be 'value'
    # name 'value' indicates to django that value of some field need to passed here
    # this will act as custom validator for specific need. Name has to start
    # with 'G'
    if value[0].lower() != 'g':
        raise forms.ValidationError("Name has to start with G")


class FormClass(forms.Form):
    name = forms.CharField(validators=[check_for_z])
    email = forms.EmailField()
    verify_email = forms.EmailField(label='Enter email again')
    text = forms.CharField(widget=forms.Textarea)
    botcatcher = forms.CharField(required=False,
                                 widget=forms.HiddenInput,
                                 validators=[validators.MaxLengthValidator(0)])

    # def clean_botcatcher(self):
    #     # name of func should start from 'clean_' and variable name
    #     botcatcher = self.cleaned_data['botcatcher']
    #     if len(botcatcher):
    #         # means some bot has entered in an otherwise invisible field
    #         # this field is not visible to on the page
    #         raise forms.ValidationError("GOTCHA BOT!")
    #     return botcatcher

    def clean(self):
        # clean data for the whole page
        all_clean_data = super().clean()
        email = all_clean_data['email']
        vmail = all_clean_data['verify_email']
        if email != vmail:
            raise forms.ValidationError("Emails not matching")


class NewUserForm(forms.ModelForm):
    class Meta:  # Name should be meta
        model = Users  # 'model should be variable name, and value should be name of model'
        fields = '__all__'
