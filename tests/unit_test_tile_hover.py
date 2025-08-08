#!/usr/bin/env python3
"""
Unit test for tile hover behavior on start screen
"""

import asyncio
import os
import sys
from pathlib import Path
from pyppeteer import launch

class TileHoverTester:
    def __init__(self):
        self.browser = None
        self.page = None
        self.project_root = Path(__file__).parent.parent
        self.game_url = f"file://{self.project_root}/index.html"
        
    async def setup(self):
        """Initialize browser and page"""
        self.browser = await launch(
            headless=False,  # Set to True for CI/CD
            args=['--no-sandbox', '--disable-setuid-sandbox'],
            autoClose=False
        )
        self.page = await self.browser.newPage()
        await self.page.setViewport({'width': 1280, 'height': 800})
        
        # Capture console logs from the browser
        self.page.on('console', lambda msg: print(f"🌐 BROWSER: {msg.text}"))
        
    async def teardown(self):
        """Clean up browser"""
        if self.browser and self.browser.process.poll() is None:
            await self.browser.close()
            
    async def wait_for_selector(self, selector, timeout=5000):
        """Wait for element with error handling"""
        try:
            await self.page.waitForSelector(selector, timeout=timeout)
            return True
        except Exception as e:
            print(f"❌ Element not found: {selector} - {e}")
            return False
            
    async def test_tile_hover_behavior(self):
        """Test tile hover changes description text below tiles"""
        print("🧪 Testing tile hover behavior...")
        
        try:
            # Load the page
            await self.page.goto(self.game_url)
            
            # Wait for start screen to be ready
            if not await self.wait_for_selector('#start-screen', timeout=10000):
                return False
                
            # Wait for tiles to be created
            if not await self.wait_for_selector('.question-set-tile'):
                print("❌ No question set tiles found")
                return False
                
            # Get initial description text
            initial_desc = await self.page.evaluate(
                '() => document.getElementById("game-description").textContent'
            )
            print(f"🔍 Initial description: '{initial_desc}'")
            
            if initial_desc != "Select a topic to begin.":
                print(f"❌ Expected 'Select a topic to begin.', got '{initial_desc}'")
                return False
            
            # Get all tiles
            tiles = await self.page.querySelectorAll('.question-set-tile')
            if len(tiles) < 2:
                print(f"❌ Need at least 2 tiles for testing, found {len(tiles)}")
                return False
                
            # Test hover on first tile
            print("🔍 Hovering over first tile...")
            await self.page.hover('.question-set-tile:first-child')
            await asyncio.sleep(0.3)  # Brief pause for hover effect
            
            # Check description changed
            hover_desc1 = await self.page.evaluate(
                '() => document.getElementById("game-description").textContent'
            )
            print(f"🔍 First tile hover description: '{hover_desc1}'")
            
            if hover_desc1 == initial_desc:
                print("❌ Description should change on hover")
                return False
                
            # Move to second tile
            print("🔍 Hovering over second tile...")
            await self.page.hover('.question-set-tile:nth-child(2)')
            await asyncio.sleep(0.3)
            
            # Check description changed again
            hover_desc2 = await self.page.evaluate(
                '() => document.getElementById("game-description").textContent'
            )
            print(f"🔍 Second tile hover description: '{hover_desc2}'")
            
            if hover_desc2 == hover_desc1:
                print("❌ Description should change between different tiles")
                return False
                
            # Move mouse away from tiles
            print("🔍 Moving mouse away from tiles...")
            await self.page.hover('#game-title')  # Hover over title instead
            await asyncio.sleep(0.3)
            
            # Check description reverted
            final_desc = await self.page.evaluate(
                '() => document.getElementById("game-description").textContent'
            )
            print(f"🔍 Final description after mouse away: '{final_desc}'")
            
            if final_desc != initial_desc:
                print(f"❌ Description should revert to '{initial_desc}', got '{final_desc}'")
                return False
                
            # Test clicking a tile starts the game
            print("🔍 Clicking first tile to start game...")
            await self.page.click('.question-set-tile:first-child')
            
            # Wait for game screen
            if not await self.wait_for_selector('#game-screen', timeout=5000):
                print("❌ Game screen should appear after clicking tile")
                return False
                
            # Check game screen is visible
            game_visible = await self.page.evaluate(
                '() => document.getElementById("game-screen").style.display !== "none"'
            )
            
            if not game_visible:
                print("❌ Game screen should be visible")
                return False
                
            print("✅ Tile hover behavior test passed")
            return True
            
        except Exception as e:
            print(f"❌ Tile hover test failed: {e}")
            return False
            
    async def run_test(self):
        """Run the tile hover test"""
        print("🎮 Starting Tile Hover Behavior Test")
        print("=" * 50)
        
        await self.setup()
        
        try:
            success = await self.test_tile_hover_behavior()
            
            if success:
                print("🎉 Tile hover test passed!")
                return True
            else:
                print("⚠️ Tile hover test failed.")
                return False
                
        finally:
            await self.teardown()

async def main():
    """Main test runner"""
    tester = TileHoverTester()
    success = await tester.run_test()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    asyncio.run(main())