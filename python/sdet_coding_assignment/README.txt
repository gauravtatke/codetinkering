Information: The folder "perf_output" has 10 json files which are test outputs from  performance test where different counter values are printed for a test run. testName is the query string describing what was the search question asked on the database during that test run. Various performance counter values display the duration each component took to do their part in the query execution.

Performance counters from the output files are:[
		'answerRendering.TABLE-MODE',
		'answerRendering.CHART-MODE',
		'answerMetadataRpc.callosum.postgres.duration',
		'answerMetadataRpc.duration',
		'answerDataRpc.CHART.duration',
		'answerDataRpc.CHART.callosum.postgres.duration',
		'answerDataRpc.CHART.callosum.falcon.duration',
		'answerDataRpc.HEADLINE+TABLE.duration',
		'answerDataRpc.HEADLINE+TABLE.callosum.postgres.duration',
		'answerDataRpc.HEADLINE+TABLE.callosum.falcon.duration',
		'sageRpc.duration'
		]

QUESTION: write a parser (in any programming language of your choice. e.g python), which goes through the list of json files in the perf_output folder and outputs a csv where we get a list of perf counter values in rows where each query has its associated perf counter values against the query string in that row. The last column is the time and date the test was run. This can be parsed from the name of the test run json file in the perf_output folder.

sample output files: sample_output.csv, query_file.csv

Can use a query encoding file such that query_0,query_1 correspond to the respective queries as mentioned in the query_file.csv. These are vlaues of the string in varios testName

Hint: if using python, can use python modules for parsing jsons
        
