<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateContent</name>
   <tag></tag>
   <elementGuidId>dec3bcad-7711-43bf-97cf-5aec25e3b6e4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;mutation CreateContent($input: ContentInput!) { createContent(input: $input) { id } }\&quot;,\n  \&quot;variables\&quot;: \&quot;{ \\\&quot;input\\\&quot;: { \\\&quot;chapterId\\\&quot;: \\\&quot;CHAPTER_ID\\\&quot;, \\\&quot;title\\\&quot;: \\\&quot;Intro Video\\\&quot;, \\\&quot;type\\\&quot;: \\\&quot;video\\\&quot;, \\\&quot;duration\\\&quot;: 1 } }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;mutation CreateContent($input: ContentInput!) {\n  createContent(input: $input) {\n    id\n  }\n}\n&quot;,
  &quot;displayVariables&quot;: &quot;{\n  \&quot;input\&quot;: {\n    \&quot;chapterId\&quot;: \&quot;CHAPTER_ID\&quot;,\n    \&quot;title\&quot;: \&quot;Intro Video\&quot;,\n    \&quot;type\&quot;: \&quot;video\&quot;,\n    \&quot;duration\&quot;: 1\n  }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9572afcb-3320-4ebb-86cb-5bbf7593b6bc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>sid_b2b=s%3AyC-N4RnpLGdLH2CrcktqpgOZB6ETrS7P.txuxnTQQFvdlDPzfi%2BGT4mweQXDIabcMyVDLmrrKH5A; __cf_bm=_HdloFO9toPL5jfjbjYUUp3_3.W_07aeBxTQzbgFDts-1769832188-1.0.1.1-cuE8FRwbNsNWFMmn.hM_y13itbkU.WULHPqk9hdABcHAm6I3_X7d4ZWBYUeLd9czuOnfgLSWVWCR9Lz9q9Vt7TZJqsXGfjWY.2hWmCHAg.c</value>
      <webElementGuid>4f9d8192-2d53-409d-8614-fa31a3c93e24</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://lmsb2b.do.dibimbing.id/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
