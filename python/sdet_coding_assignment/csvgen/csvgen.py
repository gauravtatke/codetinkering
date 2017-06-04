#!/usr/bin/env python3
import sys
import os
import os.path
import json
import re
import csv
# from datetime import datetime

perfcount = [
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


class ReportObj:
    def __init__(self):
        self.query = None
        self.query_alias = None
        self.ans_rend_table_mode = None
        self.ans_rend_chart_mode = None
        self.ans_metdata_rpc_cal_posgres_dur = None
        self.ans_metdata_rpc_dur = None
        self.ans_data_rpc_chart_dur = None
        self.ans_data_rpc_chart_cal_posgres_dur = None
        self.ans_data_rpc_chart_cal_falcon_dur = None
        self.ans_data_rpc_headln_table_dur = None
        self.ans_data_rpc_headln_table_cal_posgres_dur = None
        self.ans_data_rpc_headln_table_cal_falcon_dur = None
        self.sage_rpc_dur = None
        self.timestamp = None

    def __str__(self):
        str1 = "query = {0}\n\
                query_alias = {1},\n\
                answerRendering.TABLE-MODE = {2},\n\
                answerRendering.CHART-MODE = {3}\n\
                answerMetadataRpc.callosum.postgres.duration = {4}\n\
                answerMetadataRpc.duration = {5}\n\
                answerDataRpc.CHART.duration = {6}\n\
                answerDataRpc.CHART.callosum.postgres.duration = {7}\n\
                answerDataRpc.CHART.callosum.falcon.duration = {8}\n\
                answerDataRpc.HEADLINE+TABLE.duration = {9}\n\
                answerDataRpc.HEADLINE+TABLE.callosum.postgres.duration = {10}\n\
                answerDataRpc.HEADLINE+TABLE.callosum.falcon.duration = {11}\n\
                sageRpc.duration = {12}\n\
                timestamp = {13}\n"

        return str1.format(
                    self.query,
                    self.query_alias,
                    self.ans_rend_table_mode,
                    self.ans_rend_chart_mode,
                    self.ans_metdata_rpc_cal_posgres_dur,
                    self.ans_metdata_rpc_dur,
                    self.ans_data_rpc_chart_dur,
                    self.ans_data_rpc_chart_cal_posgres_dur,
                    self.ans_data_rpc_chart_cal_falcon_dur,
                    self.ans_data_rpc_headln_table_dur,
                    self.ans_data_rpc_headln_table_cal_posgres_dur,
                    self.ans_data_rpc_headln_table_cal_falcon_dur,
                    self.sage_rpc_dur,
                    self.timestamp
                    )


def main(argv):
    counterdict = {key: key.split('.') for key in perfcount}
    data = []
    with open('/home/gaurav/programs/python/sdet_coding_assignment/query_file.csv', newline='') as csvfile:
        csvread = csv.reader(csvfile, delimiter=',')
        qalias = {row[1].strip(): row[0].strip() for row in csvread}

    for root, _, files in os.walk(argv[0]):
        robj = ReportObj()
        for fname in files:
            robj.timestamp = gettimestamp(fname)
            with open(os.path.join(root, fname)) as fh:
                try:
                    qdata = json.load(fh)
                except json.decoder.JSONDecodeError as jsonerr:
                    print('not a valid json file, skipping', jsonerr)
                    continue
                else:
                    robj.query = getCounterData(qdata, ['testName']).strip()
                    robj.query_alias = qalias[robj.query]
                    robj.ans_rend_table_mode = getCounterData(
                        qdata, counterdict['answerRendering.TABLE-MODE'])
                    robj.ans_rend_chart_mode = getCounterData(
                        qdata, counterdict['answerRendering.CHART-MODE'])
                    robj.ans_metdata_rpc_cal_posgres_dur = getCounterData(
                        qdata, counterdict['answerMetadataRpc.callosum.postgres.duration'])
                    robj.ans_metdata_rpc_dur = getCounterData(
                        qdata, counterdict['answerMetadataRpc.duration'])
                    robj.ans_data_rpc_chart_dur = getCounterData(
                        qdata, counterdict['answerDataRpc.CHART.duration'])
                    robj.ans_data_rpc_chart_cal_posgres_dur = getCounterData(
                        qdata, counterdict['answerDataRpc.CHART.callosum.postgres.duration'])
                    robj.ans_data_rpc_chart_cal_falcon_dur = getCounterData(
                        qdata, counterdict['answerDataRpc.CHART.callosum.falcon.duration'])
                    robj.ans_data_rpc_headln_table_dur = getCounterData(
                        qdata, counterdict['answerDataRpc.HEADLINE+TABLE.duration'])
                    robj.ans_data_rpc_headln_table_cal_posgres_dur = getCounterData(
                        qdata, counterdict['answerDataRpc.HEADLINE+TABLE.callosum.postgres.duration'])
                    robj.ans_data_rpc_headln_table_cal_falcon_dur = getCounterData(
                        qdata, counterdict['answerDataRpc.HEADLINE+TABLE.callosum.falcon.duration'])
                    robj.sage_rpc_dur = getCounterData(
                        qdata, counterdict['sageRpc.duration'])
            data.append(robj)
            print(robj)
        #print(data)


def getCounterData(qdata, clist):
    val = qdata['e2e']
    for key in clist:
        try:
            val = val.get(key, -1)
        except AttributeError as atter:
            val = -1
            break
    return val

def gettimestamp(fname):
    m = re.match(r'([0-9_]+)-\w+', fname)
    if m:
        ts = m.group(1).split('_')
        return '/'.join(ts[:3]) + ' ' + ':'.join(ts[3:])
    return None


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
