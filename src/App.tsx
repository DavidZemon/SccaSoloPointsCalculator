import 'react-toastify/dist/ReactToastify.css';
import 'bootswatch/dist/slate/bootstrap.css';
import 'react-bootstrap-typeahead/css/Typeahead.css';
import { Component, ComponentPropsWithoutRef } from 'react';
import { Col, Container, Row } from 'react-bootstrap';
import { Typeahead } from 'react-bootstrap-typeahead';
import { ToastContainer } from 'react-toastify';
import { ChampionshipResultsParser } from './services';
import { ChampionshipResults, ChampionshipType } from './models';
import { FileUploadBox } from './components/FileUploadBox';
import {
  Driver,
  EventResults,
  LapTime,
  parse,
  ShortCarClass,
} from 'rusty/rusty';
import { convertClassResults } from './services/rust_helpers';

interface AppState {
  eventResultsFile?: File;
  championshipResultsFiles: Partial<Record<ChampionshipType, File>>;

  processing: boolean;

  eventResults?: EventResults;
  driversInError?: Driver[];
  championshipResults?: ChampionshipResults;

  newLadies: string[];
}

class App extends Component<ComponentPropsWithoutRef<any>, AppState> {
  private readonly championshipResultsProcessor =
    new ChampionshipResultsParser();

  constructor(props: Readonly<ComponentPropsWithoutRef<any>>) {
    super(props);
    this.state = {
      championshipResultsFiles: {
        Class: undefined,
        PAX: undefined,
        Novice: undefined,
        Ladies: undefined,
      },
      processing: false,
      newLadies: [],
    };
  }

  render() {
    return (
      <div>
        <ToastContainer pauseOnHover />
        <Container fluid>
          {/* Page header */}
          <Row>
            <Col>
              <h1>SCCA Solo Points Calculator</h1>
            </Col>
          </Row>

          {/* File upload boxes */}
          <Row>
            <Col>
              <FileUploadBox
                label={'Full Event Results (by class)'}
                file={this.state.eventResultsFile}
                accept={'.csv'}
                onFileSelect={async (f) => {
                  try {
                    // const js_results = parse_to_js(await f.text(), false);
                    // console.dir(js_results);
                    const results = parse(await f.text(), false); //new EventResults(js_results);
                    const ssc = convertClassResults(
                      results.get(ShortCarClass.SSC),
                    );
                    console.dir(ssc);

                    const driver = ssc!.drivers[0];
                    console.log(`Best lap: ${driver.best_lap()}`);
                    console.log(
                      `${driver.get_name()} times: ${driver.day_1_times}`,
                    );

                    return false;
                  } catch (e) {
                    console.error(e);
                    return false;
                  }
                  // try {
                  //   const eventResults = await this.eventResultsParser.parse(
                  //     await f.text(),
                  //   );
                  //   const driversInError = Object.entries(eventResults)
                  //     .map(([_, classResults]) => classResults.drivers)
                  //     .flat()
                  //     .filter((driver) => driver.error);
                  //   console.dir(driversInError);
                  //   if (driversInError.length)
                  //     this.setState({ eventResultsFile: f, driversInError });
                  //   else {
                  //     this.setState({
                  //       eventResultsFile: f,
                  //       eventResults,
                  //     });
                  //     await this.processChampionships();
                  //   }
                  //   return true;
                  // } catch (e) {
                  //   console.error(e);
                  //   toast.error(
                  //     'File format does not match expected. Please export event results with raw times, grouped by class.',
                  //   );
                  //   return false;
                  // }
                }}
                fileSelectedMessage={(f) => {
                  const elements = [
                    <p key={'resultsSummary'}>
                      Showing results for <code>{f.name}</code> as new event
                      results.
                    </p>,
                  ];
                  if (this.state.driversInError?.length) {
                    elements.push(
                      <p key={'errorIntro'}>
                        The following drivers appear to be be in an error state:
                      </p>,
                      <ul key={'errorList'}>
                        {this.state.driversInError.map((driver) => (
                          <li
                            key={`driverInError-${driver.car_number}${driver.car_class.short}`}
                          >
                            {driver.get_name()} {driver.car_number}{' '}
                            {driver.car_class.short}
                          </li>
                        ))}
                      </ul>,
                      <p key={'demandRefresh'}>
                        Please fix the errors and refresh this page.
                      </p>,
                      <p key={'fixInstructions'}>
                        To fix the errors, open TSAnnounce, search for each
                        class listed above, then re-run the export function from
                        TSAdmin
                      </p>,
                    );
                  }
                  return elements;
                }}
              />

              <Typeahead
                id={'newLadiesInput'}
                placeholder={'Names of first-time ladies championship drivers'}
                disabled={!this.state.eventResults}
                options={Object.values(this.state.eventResults || {})
                  .map((classResults) => classResults.drivers)
                  .flat()
                  .map((driver) => driver.name)}
                multiple
                onChange={(newLadies) => {
                  this.setState({ newLadies });
                }}
              />
            </Col>

            <Col>
              {Object.keys(this.state.championshipResultsFiles).map(
                (championshipType, index) => (
                  <FileUploadBox
                    key={index}
                    label={`${championshipType} Championship Standings`}
                    accept={'.xls,.xlsx'}
                    file={
                      this.state.championshipResultsFiles[
                        championshipType as ChampionshipType
                      ]
                    }
                    onFileSelect={async (f) => {
                      await this.processChampionships(
                        championshipType as ChampionshipType,
                        f,
                      );
                      return true;
                    }}
                    fileSelectedMessage={(f) =>
                      this.state.eventResultsFile ? (
                        <p>
                          Showing <strong>{championshipType}</strong>{' '}
                          championship standings based on <code>{f.name}</code>
                        </p>
                      ) : (
                        <p>
                          <code>{f.name}</code> set as{' '}
                          <strong>{championshipType}</strong> championship
                          standings. Please add event results to begin
                          processing.
                        </p>
                      )
                    }
                  />
                ),
              )}
            </Col>
          </Row>

          {/* Process button */}
          {/*<Row>*/}
          {/*  <Col>*/}
          {/*    <Button*/}
          {/*      style={{ width: '150px' }}*/}
          {/*      disabled={*/}
          {/*        Object.values(this.state.championshipResultsFiles).filter(*/}
          {/*          (v) => v,*/}
          {/*        ).length === 0*/}
          {/*      }*/}
          {/*      variant={'primary'}*/}
          {/*      onClick={async () => await this.processChampionships()}*/}
          {/*    >*/}
          {/*      {this.state.processing ? (*/}
          {/*        <Spinner animation={'border'} />*/}
          {/*      ) : (*/}
          {/*        <span>Reprocess Championship</span>*/}
          {/*      )}*/}
          {/*    </Button>*/}
          {/*  </Col>*/}
          {/*</Row>*/}

          {/*<EventResultsComponent*/}
          {/*  results={this.state.eventResults}*/}
          {/*  ladiesIds={this.state.championshipResults?.Ladies?.drivers?.map(*/}
          {/*    (driver) => driver.id,*/}
          {/*  )}*/}
          {/*/>*/}

          {/*<ChampionshipResultsComponent*/}
          {/*  results={this.state.championshipResults}*/}
          {/*/>*/}
        </Container>
      </div>
    );
  }

  private async processChampionships(
    championshipType?: ChampionshipType,
    newFile?: File,
  ): Promise<void> {
    const mergedFiles = { ...this.state.championshipResultsFiles };
    if (championshipType && newFile) {
      mergedFiles[championshipType] = newFile;
      this.setState({ championshipResultsFiles: mergedFiles });
    }
    if (this.state.eventResults) {
      this.setState({ processing: true });
      this.setState({
        processing: false,
        championshipResults: await this.championshipResultsProcessor.parse(
          mergedFiles,
          {}, //this.state.eventResults,
          this.state.newLadies,
        ),
      });
    }
  }
}

export default App;
