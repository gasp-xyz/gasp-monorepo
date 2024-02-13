{{/*
Useful links:
https://itnext.io/helm-reusable-chart-named-templates-and-a-generic-chart-for-multiple-applications-13d9b26e9244
https://www.replex.io/blog/9-best-practices-and-examples-for-working-with-kubernetes-labels
*/}}

{{/*
Expand the name of the chart.
*/}}
{{- define "rollup-updater.name" -}}
{{- default .Release.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "rollup-updater.labels" -}}
{{ include "rollup-updater.selectorLabels" . }}
heritage: {{ .Release.Service }}
component: rollup-eigen-layer
{{- end }}

{{/*
Selector labels
*/}}
{{- define "rollup-updater.selectorLabels" -}}
application: {{ include "rollup-updater.name" . }}
{{- end }}