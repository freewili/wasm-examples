/* Basic game code is released as GPL by Giovanni Verrua here: https://www.hackster.io/giobbino/simon-clone-with-arduino-nano-9edb51
 *  Ported to Freewili WASM by William Skellenger (wskellenger@intrepidcs.com) October 2024
 */

#include "fwwasm.h"
#include <array>

#define MAX_LOOPS 20
#define NUM_LEDS 7
#define DELAY_MS 50
#define LED_FADE_DURATION 300

// LED colors
#define COLOR_RED 0x990000
#define COLOR_YELLOW 0x999900
#define COLOR_GREEN 0x009900
#define COLOR_BLUE 0x000099

#define GET_RED(x) ((x >> 16) & 0xFF)
#define GET_GREEN(x) ((x >> 8) & 0xFF)
#define GET_BLUE(x) (x & 0xFF)

// LED color positions correspond to color buttons on Freewili
#define LED_RED 5
#define LED_YEL 2
#define LED_GRE 3
#define LED_BLU 4

#define TIMEOUT 5000 // Timeout 5 Seconds for the player to hit next light before to loose the game - Same to the original 1978 Simon
#define GAMESPD 500 // milliseconds for every led/sound

#define NOTE_E2 82.41f
#define NOTE_A2 110.0f
#define NOTE_E3 164.81f
#define NOTE_CS3 138.59f
#define NOTE_A3 220.0f
#define NOTE_C4 261.63f
#define NOTE_E4 329.63f
#define NOTE_G4 392.0f

#define RED_IDX 0
#define YEL_IDX 1
#define GRN_IDX 2
#define BLU_IDX 3
#define GRA_IDX 4

#define BUT_NONE -1

#define SPEEDUP_INCREMENT 10 // milliseconds to speed up with each level

#define LEVEL_1 0
#define LEVEL_2 1
#define LEVEL_3 2
#define LEVEL_4 3
#define L1_GUESSES 8
#define L2_GUESSES 14
#define L3_GUESSES 20
#define L4_GUESSES 31
#define NUM_LEVELS 4

struct GameButton
{
	float tone_freq;
	uint32_t led_color;
	uint8_t index;
	uint8_t led_position;
	uint8_t button_event;
};

uint8_t LEVEL_GUESSES[NUM_LEVELS] = { L1_GUESSES, L2_GUESSES, L3_GUESSES, L4_GUESSES };
uint8_t gameLevel = LEVEL_1;

volatile uint8_t statGame = 0; // 0 = idle, 1 = game running
GameButton game_buttons[4];
uint8_t gamSeq[L4_GUESSES]; // no need to initialize this array
uint8_t game_index = 0; // game pointer (sequence)
int speedup = 0; // number of ms to decrease guessing time by
uint8_t exitApp = 0;

uint8_t get_random() // range 0-3
{
	// wilirand() has the same seed so attempt to adjust it with millis()
	unsigned int result = (unsigned int)wilirand() + millis();
	return ((result >> 2) & 0x03);
}

//play light and sound at a specified duration
void light_and_sound(int index, float duration, int wait_ms)
{
	setBoardLED(game_buttons[index].led_position, GET_RED(game_buttons[index].led_color), GET_GREEN(game_buttons[index].led_color),
		GET_BLUE(game_buttons[index].led_color), GAMESPD, LEDManagerLEDMode::ledpulsefade);

	playSoundFromFrequencyAndDuration(game_buttons[index].tone_freq, duration, 0.2f, 1);
	waitms(wait_ms);
}

//play light and sound with the game's duration and wait settings
void light_and_sound_game(uint8_t index)
{
	int gamespeed_adjusted = GAMESPD;
	gamespeed_adjusted -= speedup;

	light_and_sound(index, (float)gamespeed_adjusted / 1000.0f, gamespeed_adjusted + (gamespeed_adjusted / 2));
}

int getButtonPress()
{
	int retval = BUT_NONE;
	uint8_t event_data[FW_GET_EVENT_DATA_MAX] = { 0 };
	int last_event = getEventData(event_data);
	for (int i = 0; i < 4; i++)
	{
		if (last_event == game_buttons[i].button_event)
		{
			retval = game_buttons[i].index;
		}
	}
	if (last_event == FWGUI_EVENT_GRAY_BUTTON)
	{
		retval = GRA_IDX;
	}
	return retval;
}

void set_ingame_menu()
{
	setPanelMenuText(0, 0, "Start");
	setPanelMenuText(0, 1, "Yel");
	setPanelMenuText(0, 2, "Grn");
	setPanelMenuText(0, 3, "Blu");
	setPanelMenuText(0, 4, "Red");
}

void set_outofgame_menu()
{
	setPanelMenuText(0, 0, "Start");
	waitms(1);

	if (gameLevel == LEVEL_1)
	{
		setPanelMenuText(0, 1, "Lvl1");
	}
	else if (gameLevel == LEVEL_2)
	{
		setPanelMenuText(0, 1, "Lvl2");
	}
	else if (gameLevel == LEVEL_3)
	{
		setPanelMenuText(0, 1, "Lvl3");
	}
	else if (gameLevel == LEVEL_4)
	{
		setPanelMenuText(0, 1, "Lvl4");
	}
	else
	{
		gameLevel = LEVEL_1;
		setPanelMenuText(0, 1, "Lvl1");
	}
	waitms(1);

	setPanelMenuText(0, 2, "-");
	waitms(1);

	setPanelMenuText(0, 3, "-");
	waitms(1);

	setPanelMenuText(0, 4, "Exit");
	waitms(1);
}

void setup()
{
	game_buttons[0].tone_freq = NOTE_A3;
	game_buttons[0].led_color = COLOR_RED;
	game_buttons[0].led_position = LED_RED;
	game_buttons[0].button_event = FWGUI_EVENT_RED_BUTTON;
	game_buttons[0].index = RED_IDX;

	game_buttons[1].tone_freq = NOTE_A2;
	game_buttons[1].led_color = COLOR_YELLOW;
	game_buttons[1].led_position = LED_YEL;
	game_buttons[1].button_event = FWGUI_EVENT_YELLOW_BUTTON;
	game_buttons[1].index = YEL_IDX;

	game_buttons[2].tone_freq = NOTE_CS3;
	game_buttons[2].led_color = COLOR_GREEN;
	game_buttons[2].led_position = LED_GRE;
	game_buttons[2].button_event = FWGUI_EVENT_GREEN_BUTTON;
	game_buttons[2].index = GRN_IDX;

	game_buttons[3].tone_freq = NOTE_E3;
	game_buttons[3].led_color = COLOR_BLUE;
	game_buttons[3].led_position = LED_BLU;
	game_buttons[3].button_event = FWGUI_EVENT_BLUE_BUTTON;
	game_buttons[3].index = BLU_IDX;

	addPanel(0, 1, 0, 0, 0, 0xFF, 0xAA, 0xCC, 1);
	set_outofgame_menu();
	showPanel(0);

	speedup = 0;

	//setCanDisplayReactToButtons(0);
}

bool demo()
{
	bool retval = false;
	int button = BUT_NONE;

	set_outofgame_menu();

	//random lights while sitting at idle
	game_index = get_random();

	waitms(10);
	setBoardLED(game_buttons[game_index].led_position, GET_RED(game_buttons[game_index].led_color),
		GET_GREEN(game_buttons[game_index].led_color), GET_BLUE(game_buttons[game_index].led_color), 400, LEDManagerLEDMode::ledpulsefade);
	waitms(400);

	if (hasEvent())
	{
		button = getButtonPress();
	}

	if (button == GRA_IDX)
	{
		speedup = 0;
		retval = true;
	}
	if (button == YEL_IDX) //make this an elseif -- poof
	{
		gameLevel++;
		if (gameLevel > LEVEL_4)
			gameLevel = LEVEL_1;
		set_outofgame_menu();
	}

	return retval;
}

bool gameplay()
{
	uint8_t playGame = 0; // 0 = computer turn, 1 = player turn
	bool gameOver = false;
	unsigned long timeout = millis(); // tracking the game timeout

	//--------------------------------------------------------------------------------\-
	if (playGame == 0)
	{ // computer turn
		uint8_t rndValue = get_random(); // 0 to 3

		gamSeq[game_index] = rndValue; // assign the latest color to the array
		speedup += SPEEDUP_INCREMENT;

		for (uint8_t i = 0; i <= game_index; i++)
		{
			light_and_sound_game(gamSeq[i]); // playing the whole sequence
			timeout = millis(); // reset timeout
		}

		playGame = 1;
	}

	if (playGame == 1)
	{ // player turn

		uint8_t j = 0;
		while (j <= game_index && !gameOver)
		{
			waitms(33);
			if (timeout + TIMEOUT < millis())
			{
				gameOver = true;
				j = game_index + 1;
			} // game timeout

			int button = BUT_NONE;
			if (hasEvent())
			{
				button = getButtonPress();
			}

			if (button != BUT_NONE)
			{ // the player pressed a button, let me light the LED and see if it's the right one!
				timeout = millis(); // resetting the timeout
				light_and_sound_game(button); // lighting the led for the pressed button
				if (button == gamSeq[j])
				{
					j++; // the player chose the right color, step ahead until the end of the current sequence
				}
				else
				{
					gameOver = true;
				} // wrong color, sorry!
			}
		}

		playGame = 0;

		if (!gameOver)
			game_index++;
	}

	return gameOver;
}

void mainloop()
{
	bool gameOver = false; // true if the player lost the game

	game_index = 0;

	//-----------------------------------------------------------------------------------------------------idle
	while (!statGame)
	{
		if (demo())
		{
			statGame = 1;
		}
	}

	game_index = 0;

	//-----------------------------------------------------------------------------------------------------IN GAME
	while (statGame == 1)
	{ // IN GAME

		set_ingame_menu();
		while ((game_index < LEVEL_GUESSES[gameLevel]) && !gameOver)
		{
			gameOver = gameplay();
		}

		//---- game is over; let me see who's the winner! ;-) ----------------------------------\-
		statGame = 0;
		waitms(1000);
		if (!gameOver)
		{ // the player won the game: playing the victory jingle ;D
			light_and_sound(gamSeq[game_index - 1], 0.1f, 120);
			light_and_sound(gamSeq[game_index - 1], 0.1f, 120);
			//light_and_sound(gamSeq[game_index-1], 0.1f, 120);  //need to enable
		}
		else
		{
			playSoundFromFrequencyAndDuration(80.0f, 2.0f, 0.2f, 1);
			waitms(2000);
		}
		//--------------------------------------------------------------------------------------/-
	}
}

int main()
{
	setup();
	while (!exitApp)
	{
		mainloop();
	}
	//setCanDisplayReactToButtons(1);
}