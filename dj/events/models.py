from django.db import models


# Create your models here.

class Event(models.Model):
    ekind = models.CharField(max_length=127)
    edata = models.JSONField(default=dict)
    created_on = models.DateTimeField(auto_now_add=True)

    class Meta:
        db_table = "events"


# ekind
# 1. page_visit
