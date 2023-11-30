{{/*
Useful links:
https://itnext.io/helm-reusable-chart-named-templates-and-a-generic-chart-for-multiple-applications-13d9b26e9244
https://www.replex.io/blog/9-best-practices-and-examples-for-working-with-kubernetes-labels
*/}}

{{/*
Expand the name of the chart.
*/}}
{{- define "node.name" -}}
{{- default .Release.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "node.labels" -}}
{{ include "node.selectorLabels" . }}
heritage: {{ .Release.Service }}
role: {{ .Values.role }}
component: parachain-node
{{- end }}

{{/*
Selector labels
*/}}
{{- define "node.selectorLabels" -}}
application: {{ include "node.name" . }}
{{- end }}