{{/*
Useful links:
https://itnext.io/helm-reusable-chart-named-templates-and-a-generic-chart-for-multiple-applications-13d9b26e9244
https://www.replex.io/blog/9-best-practices-and-examples-for-working-with-kubernetes-labels
*/}}

{{/*
Expand the name of the chart.
*/}}
{{- define "mangata-updater.name" -}}
{{- default .Release.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "mangata-updater.labels" -}}
{{ include "mangata-updater.selectorLabels" . }}
heritage: {{ .Release.Service }}
component: mangata-eigen-layer
{{- end }}

{{/*
Selector labels
*/}}
{{- define "mangata-updater.selectorLabels" -}}
application: {{ include "mangata-updater.name" . }}
{{- end }}