{{/*
Useful links:
https://itnext.io/helm-reusable-chart-named-templates-and-a-generic-chart-for-multiple-applications-13d9b26e9244
https://www.replex.io/blog/9-best-practices-and-examples-for-working-with-kubernetes-labels
*/}}

{{/*
Expand the name of the chart.
*/}}
{{- define "mangata-sequencer.name" -}}
{{- default .Release.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "mangata-sequencer.labels" -}}
{{ include "mangata-sequencer.selectorLabels" . }}
heritage: {{ .Release.Service }}
component: mangata-eigen-layer
{{- end }}

{{/*
Selector labels
*/}}
{{- define "mangata-sequencer.selectorLabels" -}}
application: {{ include "mangata-sequencer.name" . }}
{{- end }}