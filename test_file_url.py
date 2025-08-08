#!/usr/bin/env python3
"""
Simple test to verify the packed HTML file works with file:// URL
"""

import asyncio
import os
from pathlib import Path
from pyppeteer import launch

async def test_file_url():
    """Test opening the packed HTML file directly via file:// URL"""
    
    project_root = Path(__file__).parent
    html_file = project_root / "index.html"
    file_url = f"file://{html_file.absolute()}"
    
    print(f"🧪 Testing file URL: {file_url}")
    
    browser = None
    try:
        # Launch browser in non-headless mode so we can see it work
        browser = await launch(
            headless=False,
            args=['--no-sandbox', '--disable-setuid-sandbox'],
            autoClose=False
        )
        
        page = await browser.newPage()
        await page.setViewport({'width': 1280, 'height': 800})
        
        print("🌐 Opening packed HTML file...")
        await page.goto(file_url)
        
        # Wait for the game to load
        print("⏳ Waiting for game to load...")
        await page.waitForSelector('#start-screen', timeout=10000)
        
        # Check that start screen is visible
        start_visible = await page.evaluate(
            'document.getElementById("start-screen").style.display !== "none"'
        )
        
        if start_visible:
            print("✅ Game loaded successfully via file:// URL!")
            
            # Check that embedded data is available
            has_data = await page.evaluate('!!window.EMBEDDED_GAME_DATA')
            if has_data:
                print("✅ Embedded game data is available")
                
                # Get question count
                question_count = await page.evaluate(
                    'window.EMBEDDED_GAME_DATA.questions.length'
                )
                print(f"✅ Found {question_count} questions in embedded data")
                
                # Test clicking start button
                print("🎮 Testing start button...")
                await page.click('#start-button')
                
                # Wait for game screen
                await page.waitForSelector('#game-screen', timeout=5000)
                game_visible = await page.evaluate(
                    'document.getElementById("game-screen").style.display !== "none"'
                )
                
                if game_visible:
                    print("✅ Game starts correctly!")
                    
                    # Get the first question
                    question_text = await page.evaluate(
                        'document.getElementById("question-text").textContent'
                    )
                    print(f"✅ First question loaded: {question_text[:50]}...")
                    
                    # Count choice buttons
                    choice_count = await page.evaluate(
                        'document.querySelectorAll(".choice-button").length'
                    )
                    print(f"✅ Found {choice_count} choice buttons")
                    
                    print("\n🎉 SUCCESS: Packed HTML file works perfectly with file:// URL!")
                    print("🎯 The game is now truly standalone and can be opened directly")
                    
                else:
                    print("❌ Game screen not visible after clicking start")
                    
            else:
                print("❌ Embedded game data not found")
                
        else:
            print("❌ Start screen not visible")
            
        # Keep browser open for manual inspection
        print("\n🔍 Browser will stay open for 10 seconds for manual inspection...")
        await asyncio.sleep(10)
        
    except Exception as e:
        print(f"❌ Error: {e}")
    finally:
        if browser:
            await browser.close()

if __name__ == "__main__":
    asyncio.run(test_file_url())