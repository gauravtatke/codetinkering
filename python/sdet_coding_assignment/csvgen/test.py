#!/usr/bin/env python3
import sys
import os
import os.path
import json

from datetime import datetime

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


def main(argv):
    counterdict = {key: key.split('.') for key in perfcount}
    data = []
    for root, _, files in os.walk(argv[0]):
        robj = ReportObj()
        for fname in files:
            #robj.timestamp = gettimestamp(fname)
            with open(os.path.join(root, fname)) as fh:
                qdata = json.load(fh)
                robj.query = qdata['testName']
                robj.ans_rend_table_mode = getCounterData(
    qdata, counterdict['answerRendering.TABLE-MODE'])

    robj.ans_rend_chart_mode = getCounterData(
        qdata, counterdict['answerRendering.CHART-MODE'])

                robj.ans_metdata_rpc_cal_posgres_dur = getCounterData(qdata,
                                                                      counterdict['answerMetadataRpc.callosum.\
                                            postgres.duration'])
                robj.ans_metdata_rpc_dur = getCounterData(qdata,
                                                          counterdict['ans_metdata_rpc_dur'])
                robj.ans_data_rpc_chart_dur = getCounterData(qdata,
                                                             counterdict['answerDataRpc.CHART.duration'])
                robj.ans_data_rpc_chart_cal_posgres_dur = \
                    getCounterData(qdata,
                                   counterdict['answerDataRpc.CHART.callosum.\
                                    postgres.duration'])
                robj.ans_data_rpc_chart_cal_falcon_dur = getCounterData(qdata,
                                                                        counterdict['answerDataRpc.CHART.callosum.\
                                            falcon.duration'])
                robj.ans_data_rpc_headln_table_dur = getCounterData(qdata,
                    counterdict['answerDataRpc.HEADLINE+TABLE.duration'])
                robj.ans_data_rpc_headln_table_cal_posgres_dur = \
                    getCounterData(qdata,
                                   counterdict['answerDataRpc.HEADLINE+TABLE.\
                                            callosum.postgres.duration'])
                robj.ans_data_rpc_headln_table_cal_falcon_dur = \
                    getCounterData(qdata,
                                   counterdict['answerDataRpc.HEADLINE+TABLE.\
                                            callosum.falcon.duration'])
                robj.sage_rpc_dur = getCounterData(qdata,
                                                   counterdict['sageRpc.duration'])
            data.append(robj)

        print(data)


def getCounterData(qdata, clist):
    val = qdata['e2e']
    for key in clist:
        try:
            val = val.get('key', -1)
        except AttributeError as atter:
            val = -1
            break
    return val


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
