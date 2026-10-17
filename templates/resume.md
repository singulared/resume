# {{ personal.name }}

> {{ personal.tagline }}

{{ personal.summary }}

- **Email:** [{{ personal.email }}](mailto:{{ personal.email }})
- **Telegram:** [{{ personal.telegram|telegram }}](https://{{ personal.telegram|telegram }})
- **GitHub:** [{{ personal.github|host }}]({{ personal.github }})
- **LinkedIn:** [{{ personal.linkedin|host }}]({{ personal.linkedin }})
- **Phone:** {{ personal.phone }}

**Core:** {{ personal.skills.join(", ") }}

---

## Experience

{% for job in work -%}
### {{ job.company }} — {{ job.role }}

*{{ job.period|period }}* · [{{ job.url|host }}]({{ job.url }})

{{ job.description }}

{% for project in job.projects -%}
#### {{ project.name }}{% if project.ongoing %} *(experimental)*{% endif %}

{{ project.description|md }}

{% for role in project.roles -%}
{% if project.roles.len() > 1 && !role.period.is_empty() -%}
*{{ role.period|period }}*

{% endif -%}
{{ role.doc|md }}

{% endfor -%}
{% if !project.technologies.is_empty() -%}
**Stack:** {{ project.technologies.join(", ") }}

{% endif -%}
{% endfor -%}
---

{% endfor %}
## Open Source

{% for project in opensource -%}
### [{{ project.name }}]({{ project.url }}) — {{ project.role }}{% let stars = project.url|stars %}{% if !stars.is_empty() %} · ★ {{ stars }}{% endif %}

{% if !project.language.is_empty() -%}*{{ project.language }}*

{% endif -%}
{{ project.description|md }}

{% endfor -%}
---

## Conferences & Speaking

{% for conf in conferences -%}
### {{ conf.name }} — {{ conf.role }}

*{{ conf.year|period }}*{% if !conf.url.is_empty() %} · [{{ conf.url|host }}]({{ conf.url }}){% endif %}

{{ conf.description|md }}

{% endfor -%}
---

## Education

{% for edu in education -%}
### {{ edu.institution }}

**{{ edu.degree }}** — {{ edu.field }}

*{{ edu.period|period }}*

{{ edu.description|md }}

{% endfor -%}
{% if !publications.is_empty() -%}
---

## Publications

{% for publication in publications -%}
### [{{ publication.title }}]({{ publication.url }})

*{{ publication.year }}*

{{ publication.description|md }}

{% endfor -%}
{% endif -%}
{% if !interests.is_empty() -%}
---

## Interests

{% for interest in interests -%}
- **{{ interest.name }}** — {{ interest.description }}
{% endfor -%}
{% endif %}
