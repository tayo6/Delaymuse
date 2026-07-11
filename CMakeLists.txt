cmake_minimum_required(VERSION 3.22)
project(KnobMuse VERSION 0.1.0)

add_subdirectory(JUCE)

juce_add_plugin(KnobMuse
    COMPANY_NAME "tayo6"
    PLUGIN_MANUFACTURER_CODE Tay6
    PLUGIN_CODE Kmse
    FORMATS VST3 Standalone
    ProductName "KnobMuse"
    VST3_CATEGORIES Tools
)

target_sources(KnobMuse PRIVATE
    Source/PluginProcessor.cpp
    Source/PluginEditor.cpp
)

target_compile_definitions(KnobMuse PUBLIC
    JUCE_WEB_BROWSER=0
    JUCE_USE_CURL=0
    JUCE_VST3_CAN_REPLACE_VST2=0
)

target_link_libraries(KnobMuse PRIVATE
    juce::juce_audio_utils
    juce::juce_audio_processors
    juce::juce_gui_basics
    juce::juce_dsp
    juce::juce_graphics
)