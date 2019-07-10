<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Subtract Calculator</name>
   <tag></tag>
   <elementGuidId>dfd7e4d0-f862-4dc1-b17d-fceefb40f2b9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;utf-8&quot;?>
&lt;s:Envelope xmlns:s=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
  &lt;s:Body>
    &lt;Subtract xmlns=&quot;http://CalculatorService&quot;>
      &lt;n1>${n1}&lt;/n1>
      &lt;n2>${n2}&lt;/n2>
    &lt;/Subtract>
  &lt;/s:Body>
&lt;/s:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Subtract</soapServiceFunction>
   <variables>
      <defaultValue>12</defaultValue>
      <description></description>
      <id>c10d6786-4742-44dc-8ca0-229b4eb4e0d1</id>
      <masked>false</masked>
      <name>n1</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>dab0391b-4155-4350-8b69-866e0a3be889</id>
      <masked>false</masked>
      <name>n2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress>http://webservice.toscacloud.com/Soap11.svc?wsdl</wsdlAddress>
</WebServiceRequestEntity>
