# Generated by Django 4.2.1 on 2023-09-27 15:01

from django.db import migrations, models


class Migration(migrations.Migration):
    initial = True

    dependencies = []

    operations = [
        migrations.CreateModel(
            name="Event",
            fields=[
                (
                    "id",
                    models.BigAutoField(
                        auto_created=True,
                        primary_key=True,
                        serialize=False,
                        verbose_name="ID",
                    ),
                ),
                ("ekind", models.CharField(max_length=127)),
                ("edata", models.JSONField(default=dict)),
                ("created_on", models.DateTimeField(auto_now_add=True)),
            ],
        ),
    ]