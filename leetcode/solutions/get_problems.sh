get_raw_list() {
curl 'https://leetcode.com/graphql' \
-H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0' -H 'Accept: */*' \
-H 'Accept-Language: zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2' \
--compressed \
-H 'content-type: application/json' \
-H 'Connection: keep-alive' \
    --data-raw '{ "operationName": "allQuestionsRaw", "variables": {}, "query": "query allQuestionsRaw {\n  allQuestions: allQuestionsRaw{\n    difficulty\n    questionFrontendId\n    titleSlug\n    questionId\n    }\n}\n" }'
}

get_raw_list | jq '["questionFrontendId", "questionId", "titleSlug", "difficulty"] as $fields| .data.allQuestions| $fields, (.[] | [.[$fields[]]]) | @csv' -r | xsv sort -Ns 1
