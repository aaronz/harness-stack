# Wide Lines Test Fixture

{% for i in range(1, 101) %}
{% set line = "Line " + i + ": " + "x" * 500 %}
{{ line }}
{% endfor %}