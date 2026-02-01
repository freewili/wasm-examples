
// Free Wili Cookie Game Example
// wasm32-wasi-clang++ -O3 -s wasmapp.cpp -o wasmapp.wasm

#include "fwwasm.h"

unsigned char btEventData[64];


int main()
{
	int iCount =0;
    unsigned char btEventData[128];

	// turn on accelerometer events
	setSensorSettings(1, 1, 20, 0, 0, 0, 0, 0,0,0,0,0,0);

    #define COOKIE_INDEX 3
    #define HITCOUNT_INDEX 0
    #define COUNTDOWN_INDEX 2
    #define WILI_INDEX 1
	#define TITLE_INDEX 4
	addPanel(1,1,1,0,0,0,0,0,1);
	addControlNumber(1,HITCOUNT_INDEX,1, 10,50,200,3,0,255,255,255,false,0,false,0);
	addControlPicture(1,WILI_INDEX,10,120,42,1);
	addControlNumber(1, COUNTDOWN_INDEX,1, 0,120,200,3,0,0,0xFF,0,false,0,false,0);
	addControlPicture(1,COOKIE_INDEX,0,50,27,1);
    addControlText(1, TITLE_INDEX,30,0,1,1,255,255,255,"Feed Wili Cookies");

    setPanelMenuText(1,4,"home");
	setPanelMenuText(1,3,"stop");
	showPanel(1);


    int iXBall=0;
    int iYBall=120;
    bool bDirection= false;
	short iX=0;
	short iY=0;
	short iZ=0;
	short iTempC =0;
	short iTempF = 0;
	bool bMoving = false;
    bool bBallUp = false;
	bool bBallDown = true;
    bool bHit= false;
    int iHitCount = 0;

    #define COOKIE_SIZE  64
    bool bNewCookiePostion = true;
    int iLocationOfCookieX = 0;
	int iLocationOfCookieY = 0;
    int iCountDown = 5'000;


	for (iCount=0;iCount<5'000;iCount++)
    {
		iCount++;
		waitms(5);

        if (hasEvent())
        {
        	  int iEvent =  getEventData(btEventData);
      		  if (iEvent == FWGUI_EVENT_GUI_SENSOR_DATA)
	          {
      		   		 iX = btEventData[0] | (btEventData[1] << 8);
      		   		 iY = btEventData[2] | (btEventData[3] << 8);
      		   		 iZ = btEventData[4] | (btEventData[5] << 8);
      		   		 iTempC = btEventData[6] | (btEventData[7] << 8);
      		   		 iTempF = btEventData[8] | (btEventData[9] << 8);
      		   		 bMoving = (btEventData[10] & 0x1) != 0;
      		   		bBallDown=false;
      		   		bBallUp=false;
					if (iY < -20)
	                {
						bBallUp = true;
	                }
                    else if (iY > 20)
                    {
                  		bBallDown = true;
                    }
	          }

        }


        if (bBallDown)
		{
        	iYBall-=1;
        	if (iYBall < 0)
        		iYBall = 0;
        }
		if (bBallUp)
		{
			iYBall+=1;
			if (iYBall >=(240-32))
				iYBall = (240-32);

		}

		if (bDirection)
		{
			iXBall += 5;
			if (iXBall > (320-32))
            {
				bDirection = false;
				playSoundFromFrequencyAndDuration(300,
					0.05, 0.3, audioWaveType::WAVETYPE_TRIANGLE);
            }
		}
		else
		{
			iXBall -= 5;
			if (iXBall < 0)
            {
				bDirection = true;
				playSoundFromFrequencyAndDuration(300,
					0.05, 0.3, audioWaveType::WAVETYPE_TRIANGLE);
             }

		}

		int iXY =0 ;
        iXY = (iXBall) << 16;
		iXY |= iYBall;

        // hit test the cookie (is ball on cookie?)
        if ((iXBall >= iLocationOfCookieX) && (iXBall < (iLocationOfCookieX+COOKIE_SIZE)) )
        {
          if ((iYBall >= iLocationOfCookieY) && (iYBall < (iLocationOfCookieY+COOKIE_SIZE)) )
          {
                if (!bHit)
                  {
                		bHit =true;
                		iHitCount++;
                		bNewCookiePostion = true;
        				playSoundFromFrequencyAndDuration(1000,
						  0.05, 0.4, audioWaveType::WAVETYPE_TRIANGLE);
          			}
          }
          else
          {
        		bHit =false;
          }
        }
        else
        {
          	bHit =false;
        }

		setControlProperty( 1, 1, controlProperty::fwControlPropertyXY, iXY);
        if (bNewCookiePostion)
        {
        	bNewCookiePostion = false;
        	iLocationOfCookieX = wilirand() % (320 - COOKIE_SIZE);
        	iLocationOfCookieY = wilirand() % (240 - COOKIE_SIZE);
        	iXY = (iLocationOfCookieX) << 16;
        	iXY |= iLocationOfCookieY;
			setControlProperty( 1, COOKIE_INDEX, controlProperty::fwControlPropertyXY, iXY);
		}

		setControlValue(1,HITCOUNT_INDEX,iHitCount);
		setControlValue(1,COUNTDOWN_INDEX,iCountDown-iCount);

	}

	setControlProperty( 1, COOKIE_INDEX, controlProperty::fwControlPropertyVisible, 0);
	setControlProperty( 1, 1, controlProperty::fwControlPropertyXY, 0);
	playSoundFromNumber(0, iHitCount, 0,0);
	waitms(1500);

    if (iHitCount < 10)
		playSoundFromNameOrID(0,67); // shame
	else if (iHitCount < 20)
         playSoundFromNameOrID(0,24); // excelent
     else
     	playSoundFromNameOrID(0,19); // dominating

	waitms(1500);

	return 0;
}

