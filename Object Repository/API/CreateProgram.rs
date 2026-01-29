<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateProgram</name>
   <tag></tag>
   <elementGuidId>ac50b4be-809e-4e2e-87b5-5469a948cf5c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;mutation CreateProgram($input: ProgramInput!) { createProgram(input: $input) { id } }\&quot;,\n  \&quot;variables\&quot;: \&quot;{ \\\&quot;input\\\&quot;: { \\\&quot;title\\\&quot;: \\\&quot;${title}\\\&quot;, \\\&quot;type\\\&quot;: \\\&quot;training\\\&quot; } }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;mutation CreateProgram($input: ProgramInput!) {\n  createProgram(input: $input) {\n    id\n  }\n}\n&quot;,
  &quot;displayVariables&quot;: &quot;{\n  \&quot;input\&quot;: {\n    \&quot;title\&quot;: \&quot;${title}\&quot;,\n    \&quot;type\&quot;: \&quot;training\&quot;\n  }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>20638dee-b66c-43f6-b730-c4a11ebba811</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>sid_b2b=s%3A46WdX-wS_RJB_44986W89ombfm79dtxB.T%2BNL%2BEF07ccge7n1ssQ%2B4MvHq9S0t4vRmG2CmW1dDpE</value>
      <webElementGuid>2168da2b-bed8-4656-b13a-19f994cc9218</webElementGuid>
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
