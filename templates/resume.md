# {{ personal.name }}

> {{ personal.tagline }}

---

## Contact Information

- **Email:** [{{ personal.email }}](mailto:{{ personal.email }})
- **Phone:** {{ personal.phone }}
- **Telegram:** {{ personal.telegram }}
- **GitHub:** [{{ personal.github }}]({{ personal.github }})
- **LinkedIn:** [{{ personal.linkedin }}]({{ personal.linkedin }})
- **Birth Date:** {{ personal.birthdate }}

**Skills:** {{ personal.skills.join(", ") }}

---

## Work Experience

{% for job in work -%}
### {{ job.company }} - {{ job.role }}

*{{ job.period }}* | [{{ job.url }}]({{ job.url }})

{{ job.description }}

{% for project in job.projects -%}
#### {{ project.name }}

{{ project.description }}

{% if !project.achievements.is_empty() -%}
{% for highlight in project.achievements %}{{ highlight }}

{% endfor -%}
{% endif -%}
{% if !project.technologies.is_empty() -%}
**Technologies:** {{ project.technologies.join(", ") }}

{% endif -%}
{% endfor -%}
---

{% endfor %}

## Open Source Contributions

{% for project in opensource -%}
### [{{ project.name }}]({{ project.url }}) - {{ project.role }}{% if project.url|stars_badge != "" %} ![stars]({{ project.url|stars_badge }}){% endif %}

{% if !project.language.is_empty() -%}**Language:** {{ project.language }}{% endif %}

{{ project.description }}

{% endfor %}

---

## Conferences & Speaking

{% for conf in conferences -%}
### {{ conf.name }} - {{ conf.role }}

*{{ conf.year }}*{% if conf.url != "" %} | [Link]({{ conf.url }}){% endif %}

{{ conf.description }}

{% endfor %}

---

## Education

{% for edu in education -%}
### {{ edu.institution }}

**{{ edu.degree }}** in {{ edu.field }}

*{{ edu.period }}*

{{ edu.description }}

{% endfor %}
{% if !publications.is_empty() %}
---

## Publications

{% for pub in publications -%}
### [{{ pub.title }}]({{ pub.url }})

*{{ pub.year }}*

{{ pub.description }}

{% endfor %}
{% endif -%}
{% if !interests.is_empty() %}
---

## Interests

{% for interest in interests -%}
- **{{ interest.name }}** — {{ interest.description }}
{% endfor %}
{% endif %}
